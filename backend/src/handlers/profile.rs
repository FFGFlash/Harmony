// backend/src/handlers/profile.rs

use crate::AppState;
use crate::middleware::CurrentUser;
use crate::models::{FullProfile, Profile, UpdateProfileRequest};
use crate::services::ProfileService;
use crate::utils::AppResult;
use axum::{
  Extension, Json,
  extract::{Path, State},
};
use uuid::Uuid;

pub async fn get_my_profile(
  State(state): State<AppState>,
  Extension(user): Extension<CurrentUser>,
) -> AppResult<Json<FullProfile>> {
  let profile = ProfileService::get_full_profile(&state.db, user.id).await?;
  Ok(Json(profile))
}

pub async fn update_my_profile(
  State(state): State<AppState>,
  Extension(user): Extension<CurrentUser>,
  Json(req): Json<UpdateProfileRequest>,
) -> AppResult<Json<Profile>> {
  let profile = ProfileService::update_profile(&state.db, user.id, req).await?;
  Ok(Json(profile))
}

pub async fn get_user_full_profile(
  State(state): State<AppState>,
  Path(user_id): Path<Uuid>,
) -> AppResult<Json<FullProfile>> {
  let profile = ProfileService::get_full_profile(&state.db, user_id).await?;
  Ok(Json(profile))
}

pub async fn get_user_full_profile_by_username(
  State(state): State<AppState>,
  Path(username): Path<String>,
) -> AppResult<Json<FullProfile>> {
  let profile = ProfileService::get_full_profile_by_username(&state.db, &username).await?;
  Ok(Json(profile))
}
