use rocket::{
    Request,
    async_trait,
    http::Status,
    request::{FromRequest, Outcome},
};

use crate::{api::error::ApiError, app_state::AppState};

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user_id: String,
    pub username: String,
}

impl AuthenticatedUser {
    pub fn require_subject(&self, user_id: &str) -> Result<(), ApiError> {
        if self.user_id == user_id {
            return Ok(());
        }

        Err(ApiError::Forbidden(
            format!(
                "User '{}' can only access their own player resources",
                self.username
            ),
        ))
    }
}

#[async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ApiError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let Some(state) = request.rocket().state::<AppState>() else {
            return Outcome::Error((
                Status::InternalServerError,
                ApiError::Internal("App state was not registered".to_string()),
            ));
        };

        match extract_bearer_token(request) {
            Ok(token) => match state.jwt.verify_token(&token) {
                Ok(claims) => Outcome::Success(Self {
                    user_id: claims.sub,
                    username: claims.username,
                }),
                Err(error) => Outcome::Error((Status::Unauthorized, error)),
            },
            Err(error) => Outcome::Error((Status::Unauthorized, error)),
        }
    }
}

fn extract_bearer_token(request: &Request<'_>) -> Result<String, ApiError> {
    let auth_header = request.headers().get_one("Authorization").ok_or_else(|| {
        ApiError::Unauthorized("Missing Authorization header".to_string())
    })?;

    auth_header
        .strip_prefix("Bearer ")
        .map(str::to_string)
        .ok_or_else(|| ApiError::Unauthorized("Authorization header must use Bearer".to_string()))
}
