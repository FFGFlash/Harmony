use crate::AppState;
use crate::services::AuthService;
use crate::utils::{AppError, AppResult};
use crate::ws::connection::{Connection, ConnectionMap};
use axum::{
  extract::{
    Query, State,
    ws::{WebSocket, WebSocketUpgrade},
  },
  response::IntoResponse,
};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct WsQuery {
  token: String,
}

pub async fn ws_handler(
  ws: WebSocketUpgrade,
  Query(query): Query<WsQuery>,
  State(state): State<AppState>,
) -> AppResult<impl IntoResponse> {
  let claims = AuthService::verify_token(&query.token)?;

  let user_id = Uuid::parse_str(&claims.sub)
    .map_err(|_| AppError::Unauthorized("Invalid user ID in token".to_string()))?;

  tracing::info!("WebSocket upgrade request from user: {}", user_id);

  Ok(ws.on_upgrade(move |socket| handle_socket(socket, user_id, state.connections)))
}

async fn handle_socket(socket: WebSocket, user_id: Uuid, connections: ConnectionMap) {
  let connection = Connection::new(user_id, socket, connections);
  connection.handle().await;
}
