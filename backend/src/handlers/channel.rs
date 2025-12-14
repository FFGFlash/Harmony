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
  Ok(Json(ChannelResponse {
    id: channel.id,
    server_id: channel.server_id,
    name: channel.name,
    position: channel.position,
    created_at: channel.created_at,
  }))
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
  let responses = channels
    .into_iter()
    .map(|c| ChannelResponse {
      id: c.id,
      server_id: c.server_id,
      name: c.name,
      position: c.position,
      created_at: c.created_at,
    })
    .collect();
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
