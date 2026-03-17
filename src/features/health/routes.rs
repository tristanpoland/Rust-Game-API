use rocket::{Route, State, get, routes, serde::json::Json};
use serde::Serialize;

use crate::{
    api::error::ApiError,
    app_state::AppState,
};

#[derive(Debug, Serialize)]
struct HealthResponse {
    service: &'static str,
    status: &'static str,
    database: &'static str,
}

#[get("/health")]
async fn health(state: &State<AppState>) -> Result<Json<HealthResponse>, ApiError> {
    state.database.ping().await?;

    Ok(Json(HealthResponse {
        service: "card-game-api",
        status: "ok",
        database: "reachable",
    }))
}

pub fn routes() -> Vec<Route> {
    routes![health]
}
