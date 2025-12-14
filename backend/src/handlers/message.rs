use crate::AppState;
use crate::middleware::CurrentUser;
use crate::models::{CreateMessageRequest, MessageResponse};
use crate::services::{ChannelService, MessageService, ServerService};
use crate::utils::AppResult;
use axum::{
  Extension, Json,
  extract::{Path, Query, State},
};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct GetMessagesQuery {
  #[serde(default = "default_limit")]
  limit: i64,
  before: Option<Uuid>,
}

#[allow(dead_code)] // Not actually dead, just supressing warning
fn default_limit() -> i64 {
  50
}

pub async fn create_message(
  State(state): State<AppState>,
  Extension(user): Extension<CurrentUser>,
  Path(channel_id): Path<Uuid>,
  Json(req): Json<CreateMessageRequest>,
) -> AppResult<Json<MessageResponse>> {
  let message = MessageService::create_message(&state.db, channel_id, user.id, req).await?;
  Ok(Json(message))
}

pub async fn get_messages(
  State(state): State<AppState>,
  Extension(user): Extension<CurrentUser>,
  Path(channel_id): Path<Uuid>,
  Query(query): Query<GetMessagesQuery>,
) -> AppResult<Json<Vec<MessageResponse>>> {
  // Verify user has access to this channel
  let channel = ChannelService::get_channel_by_id(&state.db, channel_id).await?;

  if !ServerService::is_member(&state.db, channel.server_id, user.id).await? {
    return Err(crate::utils::AppError::Unauthorized(
      "You are not a member of this server".to_string(),
    ));
  }

  let limit = query.limit.min(100); // Cap at 100 messages
  let messages =
    MessageService::get_channel_messages(&state.db, channel_id, limit, query.before).await?;
  Ok(Json(messages))
}
