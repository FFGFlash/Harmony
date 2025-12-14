use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct User {
  pub id: Uuid,
  pub username: String,
  pub email: String,
  #[serde(skip_serializing)]
  pub password_hash: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
  pub username: String,
  pub email: String,
  pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
  pub email: String,
  pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
  pub id: Uuid,
  pub username: String,
  pub email: String,
  pub created_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
  fn from(value: User) -> Self {
    Self {
      id: value.id,
      username: value.username,
      email: value.email,
      created_at: value.created_at,
    }
  }
}
