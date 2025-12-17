use crate::middleware::CurrentUser;
use crate::models::Friendship;
use crate::services::FriendshipService;
use crate::utils::AppResult;
use crate::{AppState, models::Profile};
use axum::{
  Extension, Json,
  extract::{Path, State},
};
use uuid::Uuid;

pub async fn create_friend_request(
  State(state): State<AppState>,
  Extension(user): Extension<CurrentUser>,
  Path(user_id): Path<Uuid>,
) -> AppResult<Json<Friendship>> {
  let friendship = FriendshipService::send_or_accept_request(&state.db, user.id, user_id).await?;
  Ok(Json(friendship))
}

pub async fn get_friends(
  State(state): State<AppState>,
  Extension(user): Extension<CurrentUser>,
) -> AppResult<Json<Vec<Profile>>> {
  let profiles = FriendshipService::get_friends(&state.db, user.id).await?;
  Ok(Json(profiles))
}

pub async fn get_incoming_requests(
  State(state): State<AppState>,
  Extension(user): Extension<CurrentUser>,
) -> AppResult<Json<Vec<Profile>>> {
  let profiles = FriendshipService::get_incoming_requests(&state.db, user.id).await?;
  Ok(Json(profiles))
}

pub async fn get_outgoing_requests(
  State(state): State<AppState>,
  Extension(user): Extension<CurrentUser>,
) -> AppResult<Json<Vec<Profile>>> {
  let profiles = FriendshipService::get_outgoing_requests(&state.db, user.id).await?;
  Ok(Json(profiles))
}
