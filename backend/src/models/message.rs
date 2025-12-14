use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Message {
  pub id: Uuid,
  pub channel_id: Uuid,
  pub user_id: Uuid,
  pub content: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateMessageRequest {
  pub content: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct MessageResponse {
  pub id: Uuid,
  pub channel_id: Uuid,
  pub user_id: Uuid,
  pub username: String,
  pub content: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}
