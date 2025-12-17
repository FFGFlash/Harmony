use crate::AppState;
use crate::middleware::CurrentUser;
use crate::models::{Friendship, FullProfile, PaginatedResponse, PaginationParams, Profile};
use crate::services::FriendshipService;
use crate::utils::AppResult;
use axum::{
  Extension, Json,
  extract::{Path, Query, State},
};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct SendFriendRequestBody {
  username: String,
}

// Send friend request by username
pub async fn send_friend_request(
  State(state): State<AppState>,
  Extension(user): Extension<CurrentUser>,
  Json(body): Json<SendFriendRequestBody>,
) -> AppResult<Json<Friendship>> {
  let recipient = FriendshipService::get_user_by_username(&state.db, &body.username).await?;
  let friendship =
    FriendshipService::send_or_accept_request(&state.db, user.id, recipient.user_id).await?;
  Ok(Json(friendship))
}

// Alternative: send by user ID
pub async fn create_friend_request(
  State(state): State<AppState>,
  Extension(user): Extension<CurrentUser>,
  Path(user_id): Path<Uuid>,
) -> AppResult<Json<Friendship>> {
  let friendship = FriendshipService::send_or_accept_request(&state.db, user.id, user_id).await?;
  Ok(Json(friendship))
}

pub async fn reject_friend_request(
  State(state): State<AppState>,
  Extension(user): Extension<CurrentUser>,
  Path(user_id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
  FriendshipService::reject_request(&state.db, user.id, user_id).await?;
  Ok(Json(
    serde_json::json!({"message": "Friend request rejected"}),
  ))
}

pub async fn remove_friend(
  State(state): State<AppState>,
  Extension(user): Extension<CurrentUser>,
  Path(user_id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
  FriendshipService::remove_friend(&state.db, user.id, user_id).await?;
  Ok(Json(
    serde_json::json!({"message": "Friend removed successfully"}),
  ))
}

pub async fn block_user(
  State(state): State<AppState>,
  Extension(user): Extension<CurrentUser>,
  Path(user_id): Path<Uuid>,
) -> AppResult<Json<Friendship>> {
  let friendship = FriendshipService::block_user(&state.db, user.id, user_id).await?;
  Ok(Json(friendship))
}

pub async fn unblock_user(
  State(state): State<AppState>,
  Extension(user): Extension<CurrentUser>,
  Path(user_id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
  FriendshipService::unblock_user(&state.db, user.id, user_id).await?;
  Ok(Json(
    serde_json::json!({"message": "User unblocked successfully"}),
  ))
}

pub async fn get_friends(
  State(state): State<AppState>,
  Extension(user): Extension<CurrentUser>,
  Query(params): Query<PaginationParams>,
) -> AppResult<Json<PaginatedResponse<FullProfile>>> {
  let params = params.sanitize();
  let profiles =
    FriendshipService::get_friends(&state.db, user.id, params.limit, params.offset).await?;
  Ok(Json(profiles))
}

pub async fn get_incoming_requests(
  State(state): State<AppState>,
  Extension(user): Extension<CurrentUser>,
  Query(params): Query<PaginationParams>,
) -> AppResult<Json<PaginatedResponse<FullProfile>>> {
  let params = params.sanitize();
  let profiles =
    FriendshipService::get_incoming_requests(&state.db, user.id, params.limit, params.offset)
      .await?;
  Ok(Json(profiles))
}

pub async fn get_outgoing_requests(
  State(state): State<AppState>,
  Extension(user): Extension<CurrentUser>,
  Query(params): Query<PaginationParams>,
) -> AppResult<Json<PaginatedResponse<FullProfile>>> {
  let params = params.sanitize();
  let profiles =
    FriendshipService::get_outgoing_requests(&state.db, user.id, params.limit, params.offset)
      .await?;
  Ok(Json(profiles))
}

pub async fn get_blocked_users(
  State(state): State<AppState>,
  Extension(user): Extension<CurrentUser>,
  Query(params): Query<PaginationParams>,
) -> AppResult<Json<PaginatedResponse<Profile>>> {
  let params = params.sanitize();
  let profiles =
    FriendshipService::get_blocked_users(&state.db, user.id, params.limit, params.offset).await?;
  Ok(Json(profiles))
}

#[derive(Deserialize)]
pub struct SearchUsersQuery {
  username: Option<String>,
  #[serde(default = "default_limit")]
  limit: i64,
  #[serde(default)]
  offset: i64,
}

fn default_limit() -> i64 {
  20
}

pub async fn search_users(
  State(state): State<AppState>,
  Query(query): Query<SearchUsersQuery>,
) -> AppResult<Json<PaginatedResponse<FullProfile>>> {
  let username = query.username.unwrap_or_default();
  if username.is_empty() {
    return Ok(Json(PaginatedResponse {
      data: vec![],
      total: 0,
      limit: query.limit,
      offset: query.offset,
      has_more: false,
    }));
  }

  let limit = query.limit.clamp(1, 50);
  let offset = query.offset.max(0);

  let users =
    FriendshipService::search_users_by_username(&state.db, &username, limit, offset).await?;
  Ok(Json(users))
}

pub async fn get_user_profile(
  State(state): State<AppState>,
  Path(user_id): Path<Uuid>,
) -> AppResult<Json<Profile>> {
  let profile = FriendshipService::get_user_profile(&state.db, user_id).await?;
  Ok(Json(profile))
}

pub async fn get_user_by_username(
  State(state): State<AppState>,
  Path(username): Path<String>,
) -> AppResult<Json<Profile>> {
  let profile = FriendshipService::get_user_by_username(&state.db, &username).await?;
  Ok(Json(profile))
}
