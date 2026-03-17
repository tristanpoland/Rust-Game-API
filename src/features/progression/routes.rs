use rocket::{Route, State, get, post, routes, serde::json::Json};

use crate::{
    api::error::ApiError,
    app_state::AppState,
    features::{
        auth::AuthenticatedUser,
        progression::{
            ClaimRewardRequest, GrantProgressRequest, ProgressionResult, ProgressionService,
            RewardInventoryItem, UnlockedCard,
        },
    },
};

#[get("/users/<user_id>/collection")]
async fn get_user_collection(
    auth: AuthenticatedUser,
    state: &State<AppState>,
    user_id: &str,
) -> Result<Json<Vec<UnlockedCard>>, ApiError> {
    auth.require_subject(user_id)?;
    let service = ProgressionService::new(&state.database);
    Ok(Json(service.list_user_cards(user_id).await?))
}

#[get("/users/<user_id>/rewards")]
async fn get_user_rewards(
    auth: AuthenticatedUser,
    state: &State<AppState>,
    user_id: &str,
) -> Result<Json<Vec<RewardInventoryItem>>, ApiError> {
    auth.require_subject(user_id)?;
    let service = ProgressionService::new(&state.database);
    Ok(Json(service.list_user_rewards(user_id).await?))
}

#[post("/users/<user_id>/progress", data = "<request>")]
async fn grant_progress(
    auth: AuthenticatedUser,
    state: &State<AppState>,
    user_id: &str,
    request: Json<GrantProgressRequest>,
) -> Result<Json<ProgressionResult>, ApiError> {
    auth.require_subject(user_id)?;
    let service = ProgressionService::new(&state.database);
    Ok(Json(service.grant_progress(user_id, request.into_inner()).await?))
}

#[post("/users/<user_id>/cards/<card_id>/unlock")]
async fn unlock_card(
    auth: AuthenticatedUser,
    state: &State<AppState>,
    user_id: &str,
    card_id: &str,
) -> Result<Json<UnlockedCard>, ApiError> {
    auth.require_subject(user_id)?;
    let service = ProgressionService::new(&state.database);
    Ok(Json(service.unlock_card(user_id, card_id).await?))
}

#[post("/users/<user_id>/rewards/<reward_id>/claim", data = "<request>")]
async fn claim_reward(
    auth: AuthenticatedUser,
    state: &State<AppState>,
    user_id: &str,
    reward_id: &str,
    request: Json<ClaimRewardRequest>,
) -> Result<Json<crate::features::catalog::RewardCatalogItem>, ApiError> {
    auth.require_subject(user_id)?;
    let service = ProgressionService::new(&state.database);
    Ok(Json(
        service
            .claim_reward(user_id, reward_id, request.into_inner())
            .await?,
    ))
}

pub fn routes() -> Vec<Route> {
    routes![
        get_user_collection,
        get_user_rewards,
        grant_progress,
        unlock_card,
        claim_reward
    ]
}
