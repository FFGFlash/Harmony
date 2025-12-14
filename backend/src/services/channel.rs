use crate::models::{
  Channel, ChannelType, CreateChannelRequest, CreateDmRequest, CreateGroupDmRequest, DmChannel,
  DmChannelResponse, DmParticipantInfo,
};
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

    let channel_type = req.channel_type.unwrap_or(ChannelType::Text);

    let max_position: Option<i32> =
      sqlx::query_scalar("SELECT MAX(position) FROM channels WHERE server_id = $1")
        .bind(server_id)
        .fetch_one(db)
        .await?;

    let position = max_position.unwrap_or(-1) + 1;

    let channel = sqlx::query_as::<_, Channel>(
      r#"
      INSERT INTO channels (server_id, name, position, channel_type, topic)
      VALUES ($1, $2, $3, $4, $5)
      RETURNING id, server_id, name, position, channel_type, topic, is_private, created_at
      "#,
    )
    .bind(server_id)
    .bind(&req.name)
    .bind(position)
    .bind(channel_type)
    .bind(&req.topic)
    .fetch_one(db)
    .await?;

    Ok(channel)
  }

  pub async fn create_dm_channel(
    db: &PgPool,
    user_id: Uuid,
    req: CreateDmRequest,
  ) -> AppResult<DmChannelResponse> {
    if user_id == req.recipient_id {
      return Err(AppError::BadRequest(
        "Cannot create DM with yourself".to_string(),
      ));
    }

    // Check if DM already exists
    let existing = sqlx::query_as::<_, (Uuid, Uuid)>(
      r#"
      SELECT dc.id, dc.channel_id
      FROM dm_channels dc
      INNER JOIN dm_participants dp1 ON dc.id = dp1.dm_channel_id
      INNER JOIN dm_participants dp2 ON dc.id = dp2.dm_channel_id
      WHERE dp1.user_id = $1 AND dp2.user_id = $2
      AND (SELECT COUNT(*) FROM dm_participants WHERE dm_channel_id = dc.id) = 2
      LIMIT 1
      "#,
    )
    .bind(user_id)
    .bind(req.recipient_id)
    .fetch_optional(db)
    .await?;

    if let Some((dm_id, channel_id)) = existing {
      return Self::get_dm_channel_response(db, dm_id, channel_id).await;
    }

    // Get recipient username
    let recipient_username: String = sqlx::query_scalar("SELECT username FROM users WHERE id = $1")
      .bind(req.recipient_id)
      .fetch_one(db)
      .await
      .map_err(|_| AppError::NotFound("Recipient not found".to_string()))?;

    let mut tx = db.begin().await?;

    // Create channel
    let channel = sqlx::query_as::<_, Channel>(
      r#"
      INSERT INTO channels (name, position, channel_type, is_private)
      VALUES ($1, 0, 'dm', true)
      RETURNING id, server_id, name, position, channel_type, topic, is_private, created_at
      "#,
    )
    .bind(format!("@{}", recipient_username))
    .fetch_one(&mut *tx)
    .await?;

    // Create DM channel entry
    let dm_channel = sqlx::query_as::<_, DmChannel>(
      r#"
      INSERT INTO dm_channels (channel_id)
      VALUES ($1)
      RETURNING id, channel_id, created_at
      "#,
    )
    .bind(channel.id)
    .fetch_one(&mut *tx)
    .await?;

    // Add participants
    sqlx::query(
      r#"
      INSERT INTO dm_participants (dm_channel_id, user_id)
      VALUES ($1, $2), ($1, $3)
      "#,
    )
    .bind(dm_channel.id)
    .bind(user_id)
    .bind(req.recipient_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Self::get_dm_channel_response(db, dm_channel.id, dm_channel.channel_id).await
  }

  pub async fn create_group_dm_channel(
    db: &PgPool,
    user_id: Uuid,
    req: CreateGroupDmRequest,
  ) -> AppResult<DmChannelResponse> {
    if req.recipient_ids.is_empty() {
      return Err(AppError::ValidationError(
        "Group DM must have at least one recipient".to_string(),
      ));
    }

    if req.recipient_ids.len() > 9 {
      return Err(AppError::ValidationError(
        "Group DM can have at most 9 recipients".to_string(),
      ));
    }

    if req.recipient_ids.contains(&user_id) {
      return Err(AppError::ValidationError(
        "Cannot include yourself in recipients list".to_string(),
      ));
    }

    let mut tx = db.begin().await?;

    // Create channel
    let channel = sqlx::query_as::<_, Channel>(
      r#"
      INSERT INTO channels (name, position, channel_type, is_private)
      VALUES ($1, 0, 'group_dm', true)
      RETURNING id, server_id, name, position, channel_type, topic, is_private, created_at
      "#,
    )
    .bind(&req.name)
    .fetch_one(&mut *tx)
    .await?;

    // Create DM channel entry
    let dm_channel = sqlx::query_as::<_, DmChannel>(
      r#"
      INSERT INTO dm_channels (channel_id)
      VALUES ($1)
      RETURNING id, channel_id, created_at
      "#,
    )
    .bind(channel.id)
    .fetch_one(&mut *tx)
    .await?;

    // Add creator
    sqlx::query(
      r#"
      INSERT INTO dm_participants (dm_channel_id, user_id)
      VALUES ($1, $2)
      "#,
    )
    .bind(dm_channel.id)
    .bind(user_id)
    .execute(&mut *tx)
    .await?;

    // Add recipients
    for recipient_id in &req.recipient_ids {
      sqlx::query(
        r#"
        INSERT INTO dm_participants (dm_channel_id, user_id)
        VALUES ($1, $2)
        "#,
      )
      .bind(dm_channel.id)
      .bind(recipient_id)
      .execute(&mut *tx)
      .await?;
    }

    tx.commit().await?;

    Self::get_dm_channel_response(db, dm_channel.id, dm_channel.channel_id).await
  }

  pub async fn get_user_dm_channels(
    db: &PgPool,
    user_id: Uuid,
  ) -> AppResult<Vec<DmChannelResponse>> {
    let dm_ids: Vec<(Uuid, Uuid)> = sqlx::query_as(
      r#"
      SELECT dc.id, dc.channel_id
      FROM dm_channels dc
      INNER JOIN dm_participants dp ON dc.id = dp.dm_channel_id
      WHERE dp.user_id = $1
      ORDER BY dc.created_at DESC
      "#,
    )
    .bind(user_id)
    .fetch_all(db)
    .await?;

    let mut results = Vec::new();
    for (dm_id, channel_id) in dm_ids {
      results.push(Self::get_dm_channel_response(db, dm_id, channel_id).await?);
    }

    Ok(results)
  }

  async fn get_dm_channel_response(
    db: &PgPool,
    dm_id: Uuid,
    channel_id: Uuid,
  ) -> AppResult<DmChannelResponse> {
    let participants = sqlx::query_as::<_, DmParticipantInfo>(
      r#"
      SELECT dp.user_id, u.username, dp.joined_at
      FROM dm_participants dp
      INNER JOIN users u ON dp.user_id = u.id
      WHERE dp.dm_channel_id = $1
      ORDER BY dp.joined_at
      "#,
    )
    .bind(dm_id)
    .fetch_all(db)
    .await?;

    let created_at = sqlx::query_scalar("SELECT created_at FROM dm_channels WHERE id = $1")
      .bind(dm_id)
      .fetch_one(db)
      .await?;

    Ok(DmChannelResponse {
      id: dm_id,
      channel_id,
      participants,
      created_at,
    })
  }

  pub async fn get_server_channels(db: &PgPool, server_id: Uuid) -> AppResult<Vec<Channel>> {
    let channels = sqlx::query_as::<_, Channel>(
      r#"
      SELECT id, server_id, name, position, channel_type, topic, is_private, created_at
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
      SELECT id, server_id, name, position, channel_type, topic, is_private, created_at
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

  pub async fn user_has_access_to_channel(
    db: &PgPool,
    channel_id: Uuid,
    user_id: Uuid,
  ) -> AppResult<bool> {
    let channel = Self::get_channel_by_id(db, channel_id).await?;

    match channel.channel_type {
      ChannelType::Dm | ChannelType::GroupDm => {
        // Check if user is a participant
        let is_participant: bool = sqlx::query_scalar(
          r#"
          SELECT EXISTS(
            SELECT 1 FROM dm_participants dp
            INNER JOIN dm_channels dc ON dp.dm_channel_id = dc.id
            WHERE dc.channel_id = $1 AND dp.user_id = $2
          )
          "#,
        )
        .bind(channel_id)
        .bind(user_id)
        .fetch_one(db)
        .await?;

        Ok(is_participant)
      }
      ChannelType::Text | ChannelType::Voice => {
        // Check if user is a server member
        if let Some(server_id) = channel.server_id {
          ServerService::is_member(db, server_id, user_id).await
        } else {
          Ok(false)
        }
      }
    }
  }

  pub async fn delete_channel(db: &PgPool, channel_id: Uuid, user_id: Uuid) -> AppResult<()> {
    let channel = Self::get_channel_by_id(db, channel_id).await?;

    match channel.channel_type {
      ChannelType::Dm | ChannelType::GroupDm => {
        return Err(AppError::BadRequest(
          "Cannot delete DM channels".to_string(),
        ));
      }
      ChannelType::Text | ChannelType::Voice => {
        if let Some(server_id) = channel.server_id {
          let server = ServerService::get_server_by_id(db, server_id).await?;
          if server.owner_id != user_id {
            return Err(AppError::Unauthorized(
              "Only the server owner can delete channels".to_string(),
            ));
          }
        } else {
          return Err(AppError::BadRequest("Invalid channel".to_string()));
        }
      }
    }

    sqlx::query("DELETE FROM channels WHERE id = $1")
      .bind(channel_id)
      .execute(db)
      .await?;

    Ok(())
  }
}
