use rocket::{Route, State, get, routes, serde::json::Json};

use crate::{
    api::error::ApiError,
    app_state::AppState,
    features::catalog::service::CatalogService,
};

#[get("/catalog/cards")]
async fn list_cards(state: &State<AppState>) -> Result<Json<Vec<super::CardCatalogItem>>, ApiError> {
    let service = CatalogService::new(&state.database);
    Ok(Json(service.list_cards().await?))
}

#[get("/catalog/rewards")]
async fn list_rewards(
    state: &State<AppState>,
) -> Result<Json<Vec<super::RewardCatalogItem>>, ApiError> {
    let service = CatalogService::new(&state.database);
    Ok(Json(service.list_rewards().await?))
}

pub fn routes() -> Vec<Route> {
    routes![list_cards, list_rewards]
}
