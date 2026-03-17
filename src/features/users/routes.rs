use rocket::{
    Route, State,
    get, post,
    response::status::Created,
    routes,
    serde::json::Json,
};

use crate::{
    api::error::ApiError,
    app_state::AppState,
    features::users::{CreateUserRequest, UserProfile, UsersService},
};

#[post("/users", data = "<request>")]
async fn create_user(
    state: &State<AppState>,
    request: Json<CreateUserRequest>,
) -> Result<Created<Json<UserProfile>>, ApiError> {
    let service = UsersService::new(&state.database);
    let user = service.create_user(request.into_inner()).await?;

    Ok(Created::new(format!("/api/users/{}", user.user_id)).body(Json(user)))
}

#[get("/users/<user_id>")]
async fn get_user(
    state: &State<AppState>,
    user_id: &str,
) -> Result<Json<UserProfile>, ApiError> {
    let service = UsersService::new(&state.database);
    Ok(Json(service.get_user(user_id).await?))
}

pub fn routes() -> Vec<Route> {
    routes![create_user, get_user]
}
