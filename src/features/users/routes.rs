use rocket::{Route, State, get, routes, serde::json::Json};

use crate::{
    api::error::ApiError,
    app_state::AppState,
    features::{auth::AuthenticatedUser, users::{UserProfile, UsersService}},
};

#[get("/users/<user_id>")]
async fn get_user(
    auth: AuthenticatedUser,
    state: &State<AppState>,
    user_id: &str,
) -> Result<Json<UserProfile>, ApiError> {
    auth.require_subject(user_id)?;
    let service = UsersService::new(&state.database);
    Ok(Json(service.get_user(user_id).await?))
}

pub fn routes() -> Vec<Route> {
    routes![get_user]
}
