use rocket::serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateUserRequest {
    pub username: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UserProfile {
    pub user_id: String,
    pub username: String,
    pub xp: i32,
    pub level: i32,
    pub created_at: String,
}
