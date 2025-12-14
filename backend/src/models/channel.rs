use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Channel {
  pub id: Uuid,
  pub server_id: Uuid,
  pub name: String,
  pub position: i32,
  pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateChannelRequest {
  pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateChannelRequest {
  pub name: Option<String>,
  pub position: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct ChannelResponse {
  pub id: Uuid,
  pub server_id: Uuid,
  pub name: String,
  pub position: i32,
  pub created_at: DateTime<Utc>,
}
