use serde::Serialize;

#[derive(Debug, Clone)]
pub struct NewUser {
    pub username: String,
    pub password_hash: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UserProfile {
    pub user_id: String,
    pub username: String,
    pub xp: i32,
    pub level: i32,
    pub created_at: String,
}

#[derive(Debug, Clone)]
pub struct StoredUserCredentials {
    pub profile: UserProfile,
    pub password_hash: Option<String>,
}
