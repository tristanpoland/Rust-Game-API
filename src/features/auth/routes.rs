use rocket::{
    Route, State,
    post,
    response::status::Created,
    routes,
    serde::json::Json,
};

use crate::{
    api::error::ApiError,
    app_state::AppState,
    features::auth::{AuthService, models::{AuthResponse, LoginRequest, RegisterRequest}},
};

#[post("/auth/register", data = "<request>")]
async fn register(
    state: &State<AppState>,
    request: Json<RegisterRequest>,
) -> Result<Created<Json<AuthResponse>>, ApiError> {
    let service = AuthService::new(&state.database, &state.jwt);
    let auth = service.register(request.into_inner()).await?;

    Ok(Created::new(format!("/api/users/{}", auth.user.user_id)).body(Json(auth)))
}

#[post("/auth/login", data = "<request>")]
async fn login(
    state: &State<AppState>,
    request: Json<LoginRequest>,
) -> Result<Json<AuthResponse>, ApiError> {
    let service = AuthService::new(&state.database, &state.jwt);
    Ok(Json(service.login(request.into_inner()).await?))
}

pub fn routes() -> Vec<Route> {
    routes![register, login]
}
