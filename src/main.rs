mod api;
mod app_state;
mod config;
mod db;
mod features;

use std::error::Error;

use app_state::AppState;
use config::AppConfig;
use db::schema::initialize_database;
use features::{auth, catalog, health, progression, users};
use rocket::{Build, Rocket};

fn build_rocket(state: AppState) -> Rocket<Build> {
    let rocket_config = state.config.rocket_config();

    rocket::custom(rocket_config)
        .manage(state)
        .mount("/", health::routes())
        .mount("/api", auth::routes())
        .mount("/api", users::routes())
        .mount("/api", catalog::routes())
        .mount("/api", progression::routes())
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = AppConfig::from_env()?;
    let database = db::Database::new(config.database.clone());
    let jwt = auth::JwtManager::new(config.auth.clone());

    initialize_database(&database, &config.bootstrap).await?;

    let state = AppState::new(config, database, jwt);
    build_rocket(state).launch().await?;

    Ok(())
}
