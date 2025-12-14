// backend/src/handlers/dm.rs
use crate::AppState;
use crate::middleware::CurrentUser;
use crate::models::{CreateDmRequest, CreateGroupDmRequest, DmChannelResponse};
use crate::services::ChannelService;
use crate::utils::AppResult;
use axum::{Extension, Json, extract::State};

pub async fn create_dm(
  State(state): State<AppState>,
  Extension(user): Extension<CurrentUser>,
  Json(req): Json<CreateDmRequest>,
) -> AppResult<Json<DmChannelResponse>> {
  let dm_channel = ChannelService::create_dm_channel(&state.db, user.id, req).await?;
  Ok(Json(dm_channel))
}

pub async fn create_group_dm(
  State(state): State<AppState>,
  Extension(user): Extension<CurrentUser>,
  Json(req): Json<CreateGroupDmRequest>,
) -> AppResult<Json<DmChannelResponse>> {
  let dm_channel = ChannelService::create_group_dm_channel(&state.db, user.id, req).await?;
  Ok(Json(dm_channel))
}

pub async fn get_user_dms(
  State(state): State<AppState>,
  Extension(user): Extension<CurrentUser>,
) -> AppResult<Json<Vec<DmChannelResponse>>> {
  let dms = ChannelService::get_user_dm_channels(&state.db, user.id).await?;
  Ok(Json(dms))
}
