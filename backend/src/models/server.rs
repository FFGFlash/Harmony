use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Server {
  pub id: Uuid,
  pub name: String,
  pub owner_id: Uuid,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateServerRequest {
  pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateServerRequest {
  pub name: String,
}

#[derive(Debug, Serialize)]
pub struct ServerResponse {
  pub id: Uuid,
  pub name: String,
  pub owner_id: Uuid,
  pub created_at: DateTime<Utc>,
  pub is_owner: bool,
}

impl Server {
  pub fn to_response(&self, current_user_id: Uuid) -> ServerResponse {
    ServerResponse {
      id: self.id,
      name: self.name.clone(),
      owner_id: self.owner_id,
      created_at: self.created_at,
      is_owner: self.owner_id == current_user_id,
    }
  }
}
