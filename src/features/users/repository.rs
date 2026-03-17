use mysql_async::prelude::Queryable;
use uuid::Uuid;

use crate::{
    api::error::ApiError,
    db::Database,
    features::users::models::{NewUser, StoredUserCredentials, UserProfile},
};

pub struct UsersRepository<'a> {
    database: &'a Database,
}

impl<'a> UsersRepository<'a> {
    pub fn new(database: &'a Database) -> Self {
        Self { database }
    }

    pub async fn create_user(&self, new_user: &NewUser) -> Result<UserProfile, ApiError> {
        let user_id = Uuid::new_v4().to_string();
        let mut client = self.database.connect().await?;

        client
            .exec_drop(
                "INSERT INTO users (user_id, username, password_hash, xp, level, created_at)
                 VALUES (?, ?, ?, 0, 1, UTC_TIMESTAMP());",
                (&user_id, &new_user.username, &new_user.password_hash),
            )
            .await?;

        self.get_user(&user_id)
            .await?
            .ok_or_else(|| ApiError::Internal("User was created but could not be reloaded".into()))
    }

    pub async fn get_user(&self, user_id: &str) -> Result<Option<UserProfile>, ApiError> {
        Ok(self
            .get_user_credentials_by_id(user_id)
            .await?
            .map(|user| user.profile))
    }

    pub async fn get_user_credentials_by_id(
        &self,
        user_id: &str,
    ) -> Result<Option<StoredUserCredentials>, ApiError> {
        let mut client = self.database.connect().await?;
        let row: Option<(String, String, Option<String>, i32, i32, String)> = client
            .exec_first(
                "SELECT user_id,
                        username,
                        password_hash,
                        xp,
                        level,
                        DATE_FORMAT(created_at, '%Y-%m-%dT%H:%i:%sZ') AS created_at
                 FROM users
                 WHERE user_id = ?;",
                (user_id,),
            )
            .await?;

        Ok(row.map(map_user_credentials))
    }

    pub async fn get_user_credentials_by_username(
        &self,
        username: &str,
    ) -> Result<Option<StoredUserCredentials>, ApiError> {
        let mut client = self.database.connect().await?;
        let row: Option<(String, String, Option<String>, i32, i32, String)> = client
            .exec_first(
                "SELECT user_id,
                        username,
                        password_hash,
                        xp,
                        level,
                        DATE_FORMAT(created_at, '%Y-%m-%dT%H:%i:%sZ') AS created_at
                 FROM users
                 WHERE username = ?;",
                (username,),
            )
            .await?;

        Ok(row.map(map_user_credentials))
    }

    pub async fn update_progress(
        &self,
        user_id: &str,
        xp: i32,
        level: i32,
    ) -> Result<UserProfile, ApiError> {
        let mut client = self.database.connect().await?;

        client
            .exec_drop(
                "UPDATE users
                 SET xp = ?, level = ?
                 WHERE user_id = ?;",
                (xp, level, user_id),
            )
            .await?;

        self.get_user(user_id).await?.ok_or_else(|| {
            ApiError::NotFound(format!("User with id '{user_id}' was not found"))
        })
    }
}

fn map_user_credentials(row: (String, String, Option<String>, i32, i32, String)) -> StoredUserCredentials {
    StoredUserCredentials {
        profile: UserProfile {
            user_id: row.0,
            username: row.1,
            xp: row.3,
            level: row.4,
            created_at: row.5,
        },
        password_hash: row.2,
    }
}
