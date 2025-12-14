use axum::{Json, extract::State};
use serde::Serialize;

use crate::{
  AppState,
  models::{CreateUserRequest, LoginRequest, UserResponse},
  services::AuthService,
  utils::AppResult,
};

#[derive(Serialize)]
pub struct AuthResponse {
  pub user: UserResponse,
  pub token: String,
}

pub async fn register(
  State(state): State<AppState>,
  Json(req): Json<CreateUserRequest>,
) -> AppResult<Json<AuthResponse>> {
  tracing::info!("Registering new user: {}", req.email);

  let user = AuthService::create_user(&state.db, req).await?;
  let token = AuthService::generate_token(user.id)?;

  Ok(Json(AuthResponse {
    user: user.into(),
    token,
  }))
}

pub async fn login(
  State(state): State<AppState>,
  Json(req): Json<LoginRequest>,
) -> AppResult<Json<AuthResponse>> {
  tracing::info!("User login attempt: {}", req.email);

  let user = AuthService::authenticate_user(&state.db, req).await?;
  let token = AuthService::generate_token(user.id)?;

  Ok(Json(AuthResponse {
    user: user.into(),
    token,
  }))
}
