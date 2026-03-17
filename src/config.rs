use std::{env, net::IpAddr, str::FromStr};

use rocket::Config as RocketConfig;

use crate::api::error::ApiError;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub host: IpAddr,
    pub port: u16,
    pub database: DatabaseConfig,
    pub bootstrap: BootstrapConfig,
    pub auth: AuthConfig,
}

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub name: String,
    pub user: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct BootstrapConfig {
    pub connect_retries: u32,
    pub connect_delay_secs: u64,
}

#[derive(Debug, Clone)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub jwt_expiration_secs: u64,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ApiError> {
        Ok(Self {
            host: parse_env("APP_HOST", "0.0.0.0")?,
            port: parse_env("APP_PORT", "8000")?,
            database: DatabaseConfig {
                host: env_or("DATABASE_HOST", "127.0.0.1"),
                port: parse_env("DATABASE_PORT", "3306")?,
                name: env_or("DATABASE_NAME", "card_game"),
                user: env_or("DATABASE_USER", "root"),
                password: env_or("DATABASE_PASSWORD", "change-me-root-password"),
            },
            bootstrap: BootstrapConfig {
                connect_retries: parse_env("DB_CONNECT_RETRIES", "20")?,
                connect_delay_secs: parse_env("DB_CONNECT_DELAY_SECS", "3")?,
            },
            auth: AuthConfig {
                jwt_secret: env_or("JWT_SECRET", "change-me-for-production"),
                jwt_expiration_secs: parse_env("JWT_EXPIRATION_SECS", "86400")?,
            },
        })
    }

    pub fn rocket_config(&self) -> RocketConfig {
        RocketConfig {
            address: self.host,
            port: self.port,
            ..RocketConfig::default()
        }
    }
}

fn env_or(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}

fn parse_env<T>(key: &str, default: &str) -> Result<T, ApiError>
where
    T: FromStr,
    T::Err: std::fmt::Display,
{
    env_or(key, default)
        .parse::<T>()
        .map_err(|error| ApiError::Config(format!("Invalid value for {key}: {error}")))
}
