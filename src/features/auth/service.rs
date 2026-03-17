use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

use crate::{
    api::error::ApiError,
    db::Database,
    features::{
        auth::{
            jwt::JwtManager,
            models::{AuthResponse, LoginRequest, RegisterRequest},
        },
        users::{NewUser, UserProfile, UsersRepository, UsersService, validate_username},
    },
};

pub struct AuthService<'a> {
    users_service: UsersService<'a>,
    users_repository: UsersRepository<'a>,
    jwt: &'a JwtManager,
}

impl<'a> AuthService<'a> {
    pub fn new(database: &'a Database, jwt: &'a JwtManager) -> Self {
        Self {
            users_service: UsersService::new(database),
            users_repository: UsersRepository::new(database),
            jwt,
        }
    }

    pub async fn register(&self, request: RegisterRequest) -> Result<AuthResponse, ApiError> {
        let username = normalize_username(&request.username);
        validate_username(&username)?;
        validate_password(&request.password)?;

        let password_hash = hash_password(&request.password)?;
        let user = self
            .users_service
            .create_user(NewUser {
                username,
                password_hash,
            })
            .await?;

        self.build_auth_response(user)
    }

    pub async fn login(&self, request: LoginRequest) -> Result<AuthResponse, ApiError> {
        let username = normalize_username(&request.username);
        validate_username(&username)?;

        let stored = self
            .users_repository
            .get_user_credentials_by_username(&username)
            .await?
            .ok_or_else(|| {
                ApiError::Unauthorized("Invalid username or password".to_string())
            })?;

        let password_hash = stored.password_hash.ok_or_else(|| {
            ApiError::Unauthorized("This account cannot log in until it is re-registered".to_string())
        })?;

        verify_password(&request.password, &password_hash)?;

        self.build_auth_response(stored.profile)
    }

    fn build_auth_response(&self, user: UserProfile) -> Result<AuthResponse, ApiError> {
        let access_token = self.jwt.issue_token(&user.user_id, &user.username)?;

        Ok(AuthResponse {
            access_token,
            token_type: "Bearer",
            expires_in_seconds: self.jwt.expiration_secs(),
            user,
        })
    }
}

fn normalize_username(username: &str) -> String {
    username.trim().to_string()
}

fn validate_password(password: &str) -> Result<(), ApiError> {
    if password.len() < 8 {
        return Err(ApiError::Validation(
            "Password must be at least 8 characters long".to_string(),
        ));
    }

    Ok(())
}

fn hash_password(password: &str) -> Result<String, ApiError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|error| ApiError::Internal(error.to_string()))
}

fn verify_password(password: &str, password_hash: &str) -> Result<(), ApiError> {
    let parsed_hash = PasswordHash::new(password_hash)
        .map_err(|_| ApiError::Unauthorized("Invalid username or password".to_string()))?;

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| ApiError::Unauthorized("Invalid username or password".to_string()))
}

#[cfg(test)]
mod tests {
    use super::validate_password;

    #[test]
    fn rejects_short_passwords() {
        let error = validate_password("short").unwrap_err();
        assert!(error.to_string().contains("at least 8"));
    }

    #[test]
    fn accepts_reasonable_passwords() {
        assert!(validate_password("hunter22").is_ok());
    }
}
