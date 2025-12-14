// backend/src/handlers/organization.rs
use crate::AppState;
use crate::middleware::CurrentUser;
use crate::models::{
  BatchUpdateServerPositionsRequest, CreateFolderRequest, OrganizedServersResponse, ServerFolder,
  ServerOrganization, UpdateFolderRequest, UpdateServerOrganizationRequest,
};
use crate::services::OrganizationService;
use crate::utils::AppResult;
use axum::{
  Extension, Json,
  extract::{Path, State},
};
use uuid::Uuid;

// Folder endpoints
pub async fn create_folder(
  State(state): State<AppState>,
  Extension(user): Extension<CurrentUser>,
  Json(req): Json<CreateFolderRequest>,
) -> AppResult<Json<ServerFolder>> {
  let folder = OrganizationService::create_folder(&state.db, user.id, req).await?;
  Ok(Json(folder))
}

pub async fn update_folder(
  State(state): State<AppState>,
  Extension(user): Extension<CurrentUser>,
  Path(folder_id): Path<Uuid>,
  Json(req): Json<UpdateFolderRequest>,
) -> AppResult<Json<ServerFolder>> {
  let folder = OrganizationService::update_folder(&state.db, folder_id, user.id, req).await?;
  Ok(Json(folder))
}

pub async fn delete_folder(
  State(state): State<AppState>,
  Extension(user): Extension<CurrentUser>,
  Path(folder_id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
  OrganizationService::delete_folder(&state.db, folder_id, user.id).await?;
  Ok(Json(
    serde_json::json!({"message": "Folder deleted successfully"}),
  ))
}

// Server organization endpoints
pub async fn update_server_organization(
  State(state): State<AppState>,
  Extension(user): Extension<CurrentUser>,
  Path(server_id): Path<Uuid>,
  Json(req): Json<UpdateServerOrganizationRequest>,
) -> AppResult<Json<ServerOrganization>> {
  let org =
    OrganizationService::update_server_organization(&state.db, server_id, user.id, req).await?;
  Ok(Json(org))
}

pub async fn batch_update_positions(
  State(state): State<AppState>,
  Extension(user): Extension<CurrentUser>,
  Json(req): Json<BatchUpdateServerPositionsRequest>,
) -> AppResult<Json<serde_json::Value>> {
  OrganizationService::batch_update_server_positions(&state.db, user.id, req).await?;
  Ok(Json(
    serde_json::json!({"message": "Server positions updated"}),
  ))
}

pub async fn get_organized_servers(
  State(state): State<AppState>,
  Extension(user): Extension<CurrentUser>,
) -> AppResult<Json<OrganizedServersResponse>> {
  let response = OrganizationService::get_organized_servers(&state.db, user.id).await?;
  Ok(Json(response))
}
