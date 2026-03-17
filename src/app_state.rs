use crate::{config::AppConfig, db::Database};

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub database: Database,
}

impl AppState {
    pub fn new(config: AppConfig, database: Database) -> Self {
        Self { config, database }
    }
}
