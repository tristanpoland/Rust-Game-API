use crate::{
    api::error::ApiError,
    db::Database,
    features::users::{
        models::{CreateUserRequest, UserProfile},
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

    pub async fn create_user(&self, request: CreateUserRequest) -> Result<UserProfile, ApiError> {
        let username = request.username.trim();

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

        match self.repository.create_user(username).await {
            Ok(user) => Ok(user),
            Err(ApiError::Database(message))
                if message.contains("UNIQUE KEY") || message.contains("duplicate key") =>
            {
                Err(ApiError::Conflict(format!(
                    "Username '{username}' is already taken"
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
