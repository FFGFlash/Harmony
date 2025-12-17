use axum::{
  Router,
  routing::{delete, get, patch, post},
};

use crate::{AppState, handlers, middleware};

// Protected api routes
pub fn routes() -> Router<AppState> {
  Router::new()
    .route("/me/profile", get(handlers::profile::get_my_profile))
    .route("/me/profile", patch(handlers::profile::update_my_profile))
    // Users
    .route("/users/search", get(handlers::friendship::search_users))
    .route(
      "/users/{user_id}",
      get(handlers::friendship::get_user_profile),
    )
    .route(
      "/users/{user_id}/profile",
      get(handlers::profile::get_user_full_profile),
    )
    .route(
      "/users/username/{username}",
      get(handlers::friendship::get_user_by_username),
    )
    .route(
      "/users/username/{username}/profile",
      get(handlers::profile::get_user_full_profile_by_username),
    )
    .route(
      "/users/{user_id}/friend",
      post(handlers::friendship::create_friend_request),
    )
    .route(
      "/users/{user_id}/friend",
      delete(handlers::friendship::remove_friend),
    )
    .route(
      "/users/{user_id}/friend/reject",
      post(handlers::friendship::reject_friend_request),
    )
    .route(
      "/users/{user_id}/block",
      post(handlers::friendship::block_user),
    )
    .route(
      "/users/{user_id}/unblock",
      delete(handlers::friendship::unblock_user),
    )
    // Friends
    .route("/friends", get(handlers::friendship::get_friends))
    .route("/friends", post(handlers::friendship::send_friend_request))
    .route(
      "/friends/incoming",
      get(handlers::friendship::get_incoming_requests),
    )
    .route(
      "/friends/outgoing",
      get(handlers::friendship::get_outgoing_requests),
    )
    .route(
      "/friends/blocked",
      get(handlers::friendship::get_blocked_users),
    )
    // Servers
    .route("/servers", post(handlers::server::create_server))
    .route("/servers", get(handlers::server::get_user_servers))
    .route("/servers/{server_id}", get(handlers::server::get_server))
    .route(
      "/servers/{server_id}",
      delete(handlers::server::delete_server),
    )
    .route(
      "/servers/{server_id}",
      patch(handlers::server::update_server),
    )
    .route(
      "/servers/{server_id}/members",
      get(handlers::server::get_server_members),
    )
    // Server Channels
    .route(
      "/servers/{server_id}/channels",
      post(handlers::channel::create_channel),
    )
    .route(
      "/servers/{server_id}/channels",
      get(handlers::channel::get_server_channels),
    )
    // Direct Message Channels
    .route("/dms", post(handlers::dm::create_dm))
    .route("/dms", get(handlers::dm::get_user_dms))
    .route("/dms/group", post(handlers::dm::create_group_dm))
    // Channels
    .route(
      "/channels/{channel_id}",
      delete(handlers::channel::delete_channel),
    )
    // Messaging
    .route(
      "/channels/{channel_id}/messages",
      post(handlers::message::create_message),
    )
    .route(
      "/channels/{channel_id}/messages",
      get(handlers::message::get_messages),
    )
    // Organization
    .route(
      "/organization/servers",
      get(handlers::organization::get_organized_servers),
    )
    .route(
      "/organization/servers/positions",
      post(handlers::organization::batch_update_positions),
    )
    .route(
      "/organization/servers/{server_id}",
      patch(handlers::organization::update_server_organization),
    )
    .route(
      "/organization/folders",
      post(handlers::organization::create_folder),
    )
    .route(
      "/organization/folders/{folder_id}",
      patch(handlers::organization::update_folder),
    )
    .route(
      "/organization/folders/{folder_id}",
      delete(handlers::organization::delete_folder),
    )
    // Auth middleware
    .layer(axum::middleware::from_fn(middleware::auth_middleware))
}
