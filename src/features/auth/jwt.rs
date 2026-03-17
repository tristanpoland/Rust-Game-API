use std::time::{SystemTime, UNIX_EPOCH};

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use rocket::serde::{Deserialize, Serialize};

use crate::{api::error::ApiError, config::AuthConfig};

#[derive(Clone)]
pub struct JwtManager {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    expiration_secs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AuthClaims {
    pub sub: String,
    pub username: String,
    pub exp: usize,
    pub iat: usize,
}

impl JwtManager {
    pub fn new(config: AuthConfig) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(config.jwt_secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(config.jwt_secret.as_bytes()),
            expiration_secs: config.jwt_expiration_secs,
        }
    }

    pub fn issue_token(&self, user_id: &str, username: &str) -> Result<String, ApiError> {
        let issued_at = current_unix_timestamp();
        let claims = AuthClaims {
            sub: user_id.to_string(),
            username: username.to_string(),
            iat: issued_at as usize,
            exp: (issued_at + self.expiration_secs) as usize,
        };

        encode(&Header::default(), &claims, &self.encoding_key).map_err(ApiError::from)
    }

    pub fn verify_token(&self, token: &str) -> Result<AuthClaims, ApiError> {
        decode::<AuthClaims>(token, &self.decoding_key, &Validation::default())
            .map(|data| data.claims)
            .map_err(|_| ApiError::Unauthorized("Invalid or expired bearer token".to_string()))
    }

    pub fn expiration_secs(&self) -> u64 {
        self.expiration_secs
    }
}

fn current_unix_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use crate::config::AuthConfig;

    use super::JwtManager;

    #[test]
    fn issues_and_verifies_tokens() {
        let jwt = JwtManager::new(AuthConfig {
            jwt_secret: "test-secret".to_string(),
            jwt_expiration_secs: 3600,
        });

        let token = jwt.issue_token("user-123", "player_one").unwrap();
        let claims = jwt.verify_token(&token).unwrap();

        assert_eq!(claims.sub, "user-123");
        assert_eq!(claims.username, "player_one");
    }
}
