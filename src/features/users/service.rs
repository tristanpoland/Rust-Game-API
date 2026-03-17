use crate::{
    api::error::ApiError,
    db::Database,
    features::users::{
        models::{NewUser, UserProfile},
        repository::UsersRepository,
    },
};

pub struct UsersService<'a> {
    repository: UsersRepository<'a>,
}

impl<'a> UsersService<'a> {
    pub fn new(database: &'a Database) -> Self {
        Self {
            repository: UsersRepository::new(database),
        }
    }

    pub async fn create_user(&self, new_user: NewUser) -> Result<UserProfile, ApiError> {
        validate_username(&new_user.username)?;

        match self.repository.create_user(&new_user).await {
            Ok(user) => Ok(user),
            Err(ApiError::Database(message))
                if message.contains("UNIQUE KEY") || message.contains("duplicate key") =>
            {
                Err(ApiError::Conflict(format!(
                    "Username '{}' is already taken",
                    new_user.username
                )))
            }
            Err(error) => Err(error),
        }
    }

    pub async fn get_user(&self, user_id: &str) -> Result<UserProfile, ApiError> {
        self.repository
            .get_user(user_id)
            .await?
            .ok_or_else(|| ApiError::NotFound(format!("User with id '{user_id}' was not found")))
    }
}

pub fn validate_username(username: &str) -> Result<(), ApiError> {
    if username.is_empty() {
        return Err(ApiError::Validation(
            "Username must not be empty".to_string(),
        ));
    }

    if username.len() > 64 {
        return Err(ApiError::Validation(
            "Username must be 64 characters or fewer".to_string(),
        ));
    }

    Ok(())
}
