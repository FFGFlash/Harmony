use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type)]
#[sqlx(type_name = "profile_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ProfileStatus {
  Online,
  Away,
  Offline,
  Dnd,
}

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

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct Profile {
  pub user_id: Uuid,
  pub display_name: Option<String>,
  pub bio: Option<String>,
  pub avatar_url: Option<String>,
  pub banner_url: Option<String>,
  pub status: ProfileStatus,
  pub custom_status: Option<String>,
  pub status_emoji: Option<String>,
  pub show_online_status: bool,
  pub allow_dms: bool,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct FullProfile {
  pub id: Uuid,
  pub username: String,
  pub display_name: Option<String>,
  pub bio: Option<String>,
  pub avatar_url: Option<String>,
  pub banner_url: Option<String>,
  pub status: ProfileStatus,
  pub custom_status: Option<String>,
  pub status_emoji: Option<String>,
  pub show_online_status: bool,
  pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProfileRequest {
  pub display_name: Option<String>,
  pub bio: Option<String>,
  pub avatar_url: Option<String>,
  pub banner_url: Option<String>,
  pub status: Option<String>,
  pub custom_status: Option<String>,
  pub status_emoji: Option<String>,
  pub show_online_status: Option<bool>,
  pub allow_dms: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
  pub username: String,
  pub email: String,
  pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
  pub username: Option<String>,
  pub email: Option<String>,
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
