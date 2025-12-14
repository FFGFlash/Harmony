// backend/src/models/organization.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::ServerResponse;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct ServerFolder {
  pub id: Uuid,
  pub user_id: Uuid,
  pub name: String,
  pub color: Option<String>,
  pub position: i32,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct ServerOrganization {
  pub user_id: Uuid,
  pub server_id: Uuid,
  pub folder_id: Option<Uuid>,
  pub position: i32,
  pub is_pinned: bool,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateFolderRequest {
  pub name: String,
  pub color: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateFolderRequest {
  pub name: Option<String>,
  pub color: Option<String>,
  pub position: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateServerOrganizationRequest {
  pub folder_id: Option<Uuid>,
  pub position: Option<i32>,
  pub is_pinned: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct BatchUpdateServerPositionsRequest {
  pub updates: Vec<ServerPositionUpdate>,
}

#[derive(Debug, Deserialize)]
pub struct ServerPositionUpdate {
  pub server_id: Uuid,
  pub position: i32,
  pub folder_id: Option<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct FolderResponse {
  pub id: Uuid,
  pub name: String,
  pub color: Option<String>,
  pub position: i32,
  pub servers: Vec<ServerResponse>,
  pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct OrganizedServersResponse {
  pub folders: Vec<FolderResponse>,
  pub ungrouped_servers: Vec<ServerResponse>,
}
