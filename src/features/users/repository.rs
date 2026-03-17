use uuid::Uuid;

use crate::{
    api::error::ApiError,
    db::Database,
    features::users::models::UserProfile,
};

pub struct UsersRepository<'a> {
    database: &'a Database,
}

impl<'a> UsersRepository<'a> {
    pub fn new(database: &'a Database) -> Self {
        Self { database }
    }

    pub async fn create_user(&self, username: &str) -> Result<UserProfile, ApiError> {
        let user_id = Uuid::new_v4().to_string();
        let mut client = self.database.connect().await?;

        client
            .execute(
                "INSERT INTO dbo.users (user_id, username, xp, level, created_at)
                 VALUES (@P1, @P2, 0, 1, SYSUTCDATETIME());",
                &[&user_id, &username],
            )
            .await?;

        self.get_user(&user_id)
            .await?
            .ok_or_else(|| ApiError::Internal("User was created but could not be reloaded".into()))
    }

    pub async fn get_user(&self, user_id: &str) -> Result<Option<UserProfile>, ApiError> {
        let mut client = self.database.connect().await?;
        let row = client
            .query(
                "SELECT user_id,
                        username,
                        xp,
                        level,
                        CONVERT(VARCHAR(33), created_at, 126) AS created_at
                 FROM dbo.users
                 WHERE user_id = @P1;",
                &[&user_id],
            )
            .await?
            .into_row()
            .await?;

        Ok(row.map(map_user))
    }

    pub async fn update_progress(
        &self,
        user_id: &str,
        xp: i32,
        level: i32,
    ) -> Result<UserProfile, ApiError> {
        let mut client = self.database.connect().await?;

        client
            .execute(
                "UPDATE dbo.users
                 SET xp = @P2, level = @P3
                 WHERE user_id = @P1;",
                &[&user_id, &xp, &level],
            )
            .await?;

        self.get_user(user_id).await?.ok_or_else(|| {
            ApiError::NotFound(format!("User with id '{user_id}' was not found"))
        })
    }
}

fn map_user(row: tiberius::Row) -> UserProfile {
    UserProfile {
        user_id: row
            .get::<&str, _>("user_id")
            .unwrap_or_default()
            .to_string(),
        username: row
            .get::<&str, _>("username")
            .unwrap_or_default()
            .to_string(),
        xp: row.get::<i32, _>("xp").unwrap_or_default(),
        level: row.get::<i32, _>("level").unwrap_or_default(),
        created_at: row
            .get::<&str, _>("created_at")
            .unwrap_or_default()
            .to_string(),
    }
}
