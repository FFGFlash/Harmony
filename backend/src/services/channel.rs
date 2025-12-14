use crate::models::{Channel, CreateChannelRequest};
use crate::services::server::ServerService;
use crate::utils::{AppError, AppResult};
use sqlx::PgPool;
use uuid::Uuid;

pub struct ChannelService;

impl ChannelService {
  pub async fn create_channel(
    db: &PgPool,
    server_id: Uuid,
    user_id: Uuid,
    req: CreateChannelRequest,
  ) -> AppResult<Channel> {
    if !ServerService::is_member(db, server_id, user_id).await? {
      return Err(AppError::Unauthorized(
        "You must be a member of the server to create channels".to_string(),
      ));
    }

    if req.name.trim().is_empty() {
      return Err(AppError::ValidationError(
        "Channel name cannot be empty".to_string(),
      ));
    }

    let max_position: Option<i32> =
      sqlx::query_scalar("SELECT MAX(position) FROM channels WHERE server_id = $1")
        .bind(server_id)
        .fetch_one(db)
        .await?;

    let position = max_position.unwrap_or(-1) + 1;

    let channel = sqlx::query_as::<_, Channel>(
      r#"
        INSERT INTO channels (server_id, name, position)
        VALUES ($1, $2, $3)
        RETURNING id, server_id, name, position, created_at
        "#,
    )
    .bind(server_id)
    .bind(&req.name)
    .bind(position)
    .fetch_one(db)
    .await?;

    Ok(channel)
  }

  pub async fn get_server_channels(db: &PgPool, server_id: Uuid) -> AppResult<Vec<Channel>> {
    let channels = sqlx::query_as::<_, Channel>(
      r#"
      SELECT id, server_id, name, position, created_at
      FROM channels
      WHERE server_id = $1
      ORDER BY position ASC
      "#,
    )
    .bind(server_id)
    .fetch_all(db)
    .await?;

    Ok(channels)
  }

  pub async fn get_channel_by_id(db: &PgPool, channel_id: Uuid) -> AppResult<Channel> {
    let channel = sqlx::query_as::<_, Channel>(
      r#"
      SELECT id, server_id, name, position, created_at
      FROM channels
      WHERE id = $1
      "#,
    )
    .bind(channel_id)
    .fetch_optional(db)
    .await?
    .ok_or_else(|| AppError::NotFound("Channel not found".to_string()))?;

    Ok(channel)
  }

  pub async fn delete_channel(db: &PgPool, channel_id: Uuid, user_id: Uuid) -> AppResult<()> {
    let channel = Self::get_channel_by_id(db, channel_id).await?;
    let server = ServerService::get_server_by_id(db, channel.server_id).await?;

    if server.owner_id != user_id {
      return Err(AppError::Unauthorized(
        "Only the server owner can delete channels".to_string(),
      ));
    }

    sqlx::query("DELETE FROM channels WHERE id = $1")
      .bind(channel_id)
      .execute(db)
      .await?;

    Ok(())
  }
}
