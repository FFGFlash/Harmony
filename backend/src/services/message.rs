use crate::models::{CreateMessageRequest, Message, MessageResponse};
use crate::services::channel::ChannelService;
use crate::utils::{AppError, AppResult};
use sqlx::PgPool;
use uuid::Uuid;

pub struct MessageService;

impl MessageService {
  pub async fn create_message(
    db: &PgPool,
    channel_id: Uuid,
    user_id: Uuid,
    req: CreateMessageRequest,
  ) -> AppResult<MessageResponse> {
    if !ChannelService::user_has_access_to_channel(db, channel_id, user_id).await? {
      return Err(AppError::Unauthorized(
        "You don't have access to that channel".to_string(),
      ));
    }

    if req.content.trim().is_empty() {
      return Err(AppError::ValidationError(
        "Message content cannot be empty".to_string(),
      ));
    }

    if req.content.len() > 200 {
      return Err(AppError::ValidationError(
        "Message cannot exceed 2000 characters".to_string(),
      ));
    }

    let message = sqlx::query_as::<_, Message>(
      r#"
      INSERT INTO messages (channel_id, user_id, content)
      VALUES ($1, $2, $3)
      RETURNING id, channel_id, user_id, content, created_at, updated_at
      "#,
    )
    .bind(channel_id)
    .bind(user_id)
    .bind(&req.content)
    .fetch_one(db)
    .await?;

    let username: String = sqlx::query_scalar("SELECT username FROM users WHERE id = $1")
      .bind(user_id)
      .fetch_one(db)
      .await?;

    let message = MessageResponse {
      id: message.id,
      channel_id: message.channel_id,
      user_id: message.user_id,
      content: message.content,
      created_at: message.created_at,
      updated_at: message.updated_at,
      username,
    };

    Ok(message)
  }

  pub async fn get_channel_messages(
    db: &PgPool,
    channel_id: Uuid,
    limit: i64,
    before: Option<Uuid>,
  ) -> AppResult<Vec<MessageResponse>> {
    let messages = if let Some(before_id) = before {
      sqlx::query_as::<_, MessageResponse>(
        r#"
        SELECT
          m.id,
          m.channel_id,
          m.user_id,
          u.username,
          m.content,
          m.created_at,
          m.updated_at
        FROM messages m
        INNER JOIN user u ON m.user_id = u.id
        WHERE m.channel_id = $1
          AND m.created_at < (SELECT created_at FROM messages WHERE id = $2)
        ORDER BY m.created_at ASC
        LIMIT $3
        "#,
      )
      .bind(channel_id)
      .bind(before_id)
      .bind(limit)
      .fetch_all(db)
      .await?
    } else {
      sqlx::query_as::<_, MessageResponse>(
        r#"
        SELECT
          m.id,
          m.channel_id,
          m.user_id,
          u.username,
          m.content,
          m.created_at,
          m.updated_at
        FROM messages m
        INNER JOIN users u ON m.user_id = u.id
        WHERE m.channel_id = $1
        ORDER BY m.created_at ASC
        LIMIT $2
        "#,
      )
      .bind(channel_id)
      .bind(limit)
      .fetch_all(db)
      .await?
    };

    Ok(messages.into_iter().rev().collect())
  }
}
