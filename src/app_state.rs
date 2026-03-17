use crate::{config::AppConfig, db::Database, features::auth::JwtManager};

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub database: Database,
    pub jwt: JwtManager,
}

impl AppState {
    pub fn new(config: AppConfig, database: Database, jwt: JwtManager) -> Self {
        Self {
            config,
            database,
            jwt,
        }
    }
}
