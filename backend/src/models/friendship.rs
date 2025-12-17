use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "friendship_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum FriendshipStatus {
  Pending,
  Accepted,
  Rejected,
  Blocked,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct Friendship {
  pub user_low: Uuid,
  pub user_high: Uuid,
  pub sender_id: Uuid,
  pub status: FriendshipStatus,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}
