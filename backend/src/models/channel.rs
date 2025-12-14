use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "channel_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ChannelType {
  Text,
  Voice,
  Dm,
  GroupDm,
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Channel {
  pub id: Uuid,
  pub server_id: Option<Uuid>,
  pub name: String,
  pub position: i32,
  pub channel_type: ChannelType,
  pub topic: Option<String>,
  pub is_private: bool,
  pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct DmChannel {
  pub id: Uuid,
  pub channel_id: Uuid,
  pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct DmParticipant {
  pub dm_channel_id: Uuid,
  pub user_id: Uuid,
  pub joined_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateChannelRequest {
  pub name: String,
  #[serde(default)]
  pub channel_type: Option<ChannelType>,
  pub topic: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateDmRequest {
  pub recipient_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct CreateGroupDmRequest {
  pub name: String,
  pub recipient_ids: Vec<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateChannelRequest {
  pub name: Option<String>,
  pub position: Option<i32>,
  pub topic: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ChannelResponse {
  pub id: Uuid,
  pub server_id: Option<Uuid>,
  pub name: String,
  pub position: i32,
  pub channel_type: ChannelType,
  pub topic: Option<String>,
  pub is_private: bool,
  pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct DmChannelResponse {
  pub id: Uuid,
  pub channel_id: Uuid,
  pub participants: Vec<DmParticipantInfo>,
  pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct DmParticipantInfo {
  pub user_id: Uuid,
  pub username: String,
  pub joined_at: DateTime<Utc>,
}

impl Channel {
  pub fn to_response(&self) -> ChannelResponse {
    ChannelResponse {
      id: self.id,
      server_id: self.server_id,
      name: self.name.clone(),
      position: self.position,
      channel_type: self.channel_type,
      topic: self.topic.clone(),
      is_private: self.is_private,
      created_at: self.created_at,
    }
  }
}
