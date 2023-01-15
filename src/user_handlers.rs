use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::Deserialize;
use validator::Validate;

use crate::{AppState, jwt};
use crate::auth_service::AuthService;

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

// TODO implement get current user
pub(crate) async fn current() -> Result<String, String> {
    todo!()
}

pub(crate) async fn login(
    State(state): State<AppState>,
    Json(input): Json<LoginRequest>,
) -> impl IntoResponse {
    match input.validate() {
        Ok(_) => (),
        Err(e) => return (StatusCode::BAD_REQUEST, e.to_string()),
    };

    let result = AuthService::sign_in((input.email, input.password), &state.conn).await;

    match result {
        Ok(model) => (StatusCode::OK, jwt::encode(model.id).unwrap()),
        Err(e) => (StatusCode::UNAUTHORIZED, e),
    }
}

pub(crate) async fn register(
    State(state): State<AppState>,
    Json(input): Json<LoginRequest>,
) -> impl IntoResponse {
    match input.validate() {
        Ok(_) => (),
        Err(e) => return (StatusCode::BAD_REQUEST, e.to_string()),
    };

    let result = AuthService::sign_up((input.email, input.password), &state.conn).await;

    match result {
        Ok(user_id) => (StatusCode::CREATED, jwt::encode(user_id).unwrap()),
        Err(e) => (StatusCode::CONFLICT, e),
    }
}
