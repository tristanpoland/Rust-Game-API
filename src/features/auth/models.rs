use rocket::serde::Deserialize;
use serde::Serialize;

use crate::features::users::UserProfile;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub token_type: &'static str,
    pub expires_in_seconds: u64,
    pub user: UserProfile,
}
