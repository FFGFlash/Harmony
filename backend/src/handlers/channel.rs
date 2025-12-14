use crate::AppState;
use crate::middleware::CurrentUser;
use crate::models::{ChannelResponse, CreateChannelRequest};
use crate::services::{ChannelService, ServerService};
use crate::utils::AppResult;
use axum::{
  Extension, Json,
  extract::{Path, State},
};
use uuid::Uuid;

pub async fn create_channel(
  State(state): State<AppState>,
  Extension(user): Extension<CurrentUser>,
  Path(server_id): Path<Uuid>,
  Json(req): Json<CreateChannelRequest>,
) -> AppResult<Json<ChannelResponse>> {
  let channel = ChannelService::create_channel(&state.db, server_id, user.id, req).await?;
  Ok(Json(channel.to_response()))
}

pub async fn get_server_channels(
  State(state): State<AppState>,
  Extension(user): Extension<CurrentUser>,
  Path(server_id): Path<Uuid>,
) -> AppResult<Json<Vec<ChannelResponse>>> {
  // Check if user is a member
  if !ServerService::is_member(&state.db, server_id, user.id).await? {
    return Err(crate::utils::AppError::Unauthorized(
      "You are not a member of this server".to_string(),
    ));
  }

  let channels = ChannelService::get_server_channels(&state.db, server_id).await?;
  let responses = channels.into_iter().map(|c| c.to_response()).collect();
  Ok(Json(responses))
}

pub async fn delete_channel(
  State(state): State<AppState>,
  Extension(user): Extension<CurrentUser>,
  Path(channel_id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
  ChannelService::delete_channel(&state.db, channel_id, user.id).await?;
  Ok(Json(
    serde_json::json!({"message": "Channel deleted successfully"}),
  ))
}
