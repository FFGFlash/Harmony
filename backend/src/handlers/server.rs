use crate::AppState;
use crate::middleware::CurrentUser;
use crate::models::{CreateServerRequest, ServerResponse, UpdateServerRequest};
use crate::services::ServerService;
use crate::utils::AppResult;
use axum::{
  Extension, Json,
  extract::{Path, State},
};
use uuid::Uuid;

pub async fn create_server(
  State(state): State<AppState>,
  Extension(user): Extension<CurrentUser>,
  Json(req): Json<CreateServerRequest>,
) -> AppResult<Json<ServerResponse>> {
  let server = ServerService::create_server(&state.db, user.id, req).await?;
  Ok(Json(server.to_response(user.id)))
}

pub async fn get_user_servers(
  State(state): State<AppState>,
  Extension(user): Extension<CurrentUser>,
) -> AppResult<Json<Vec<ServerResponse>>> {
  let servers = ServerService::get_user_servers(&state.db, user.id).await?;
  let responses = servers
    .into_iter()
    .map(|s| s.to_response(user.id))
    .collect();
  Ok(Json(responses))
}

pub async fn get_server(
  State(state): State<AppState>,
  Extension(user): Extension<CurrentUser>,
  Path(server_id): Path<Uuid>,
) -> AppResult<Json<ServerResponse>> {
  // Check if user is a member
  if !ServerService::is_member(&state.db, server_id, user.id).await? {
    return Err(crate::utils::AppError::Unauthorized(
      "You are not a member of this server".to_string(),
    ));
  }

  let server = ServerService::get_server_by_id(&state.db, server_id).await?;
  Ok(Json(server.to_response(user.id)))
}

pub async fn delete_server(
  State(state): State<AppState>,
  Extension(user): Extension<CurrentUser>,
  Path(server_id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
  ServerService::delete_server(&state.db, server_id, user.id).await?;
  Ok(Json(
    serde_json::json!({"message": "Server deleted successfully"}),
  ))
}

pub async fn update_server(
  State(state): State<AppState>,
  Extension(user): Extension<CurrentUser>,
  Path(server_id): Path<Uuid>,
  Json(req): Json<UpdateServerRequest>,
) -> AppResult<Json<ServerResponse>> {
  let server = ServerService::update_server(&state.db, server_id, user.id, req).await?;
  Ok(Json(server.to_response(user.id)))
}
