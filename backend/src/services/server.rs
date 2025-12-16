use crate::models::{CreateServerRequest, Server, UpdateServerRequest};
use crate::utils::{AppError, AppResult};
use sqlx::{Executor, PgPool, Postgres};
use uuid::Uuid;

pub struct ServerService;

impl ServerService {
  pub async fn create_server(
    db: &PgPool,
    user_id: Uuid,
    req: CreateServerRequest,
  ) -> AppResult<Server> {
    if req.name.trim().is_empty() {
      return Err(AppError::ValidationError(
        "Server name cannot be empty".to_string(),
      ));
    }

    let mut tx = db.begin().await?;

    let server = sqlx::query_as::<_, Server>(
      r#"
      INSERT INTO servers (name, owner_id)
      VALUES ($1, $2)
      RETURNING id, name, owner_id, main_channel_id, created_at, updated_at
      "#,
    )
    .bind(&req.name)
    .bind(user_id)
    .fetch_one(&mut *tx)
    .await?;

    sqlx::query(
      r#"
      INSERT INTO server_members (server_id, user_id)
      VALUES ($1, $2)
      "#,
    )
    .bind(server.id)
    .bind(user_id)
    .execute(&mut *tx)
    .await?;

    let channel_id: Uuid = sqlx::query_scalar(
      r#"
      INSERT INTO channels (server_id, name, position)
      VALUES ($1, 'general', 0)
      RETURNING id
      "#,
    )
    .bind(server.id)
    .fetch_one(&mut *tx)
    .await?;

    let server = sqlx::query_as::<_, Server>(
      r#"
      UPDATE servers
      SET main_channel_id = $1
      WHERE id = $2
      RETURNING id, name, owner_id, main_channel_id, created_at, updated_at
      "#,
    )
    .bind(channel_id)
    .bind(server.id)
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(server)
  }

  pub async fn get_user_servers(db: &PgPool, user_id: Uuid) -> AppResult<Vec<Server>> {
    let servers = sqlx::query_as::<_, Server>(
      r#"
      SELECT s.id, s.name, s.owner_id, s.main_channel_id, s.created_at, s.updated_at
      FROM servers s
      INNER JOIN server_members sm ON s.id = sm.server_id
      WHERE sm.user_id = $1
      ORDER BY s.created_at DESC
      "#,
    )
    .bind(user_id)
    .fetch_all(db)
    .await?;

    Ok(servers)
  }

  pub async fn get_server_by_id(db: &PgPool, server_id: Uuid) -> AppResult<Server> {
    let server = sqlx::query_as::<_, Server>(
      r#"
      SELECT id, name, owner_id, main_channel_id, created_at, updated_at
      FROM servers
      WHERE id = $1
      "#,
    )
    .bind(server_id)
    .fetch_optional(db)
    .await?
    .ok_or_else(|| AppError::NotFound("Server not found".to_string()))?;

    Ok(server)
  }

  pub async fn is_member<'e, E>(db: E, server_id: Uuid, user_id: Uuid) -> AppResult<bool>
  where
    E: Executor<'e, Database = Postgres>,
  {
    let result = sqlx::query_scalar::<_, bool>(
      r#"
      SELECT EXISTS(
        SELECT 1 FROM server_members
        WHERE server_id = $1 AND user_id = $2
      )
      "#,
    )
    .bind(server_id)
    .bind(user_id)
    .fetch_one(db)
    .await?;

    Ok(result)
  }

  pub async fn delete_server(db: &PgPool, server_id: Uuid, user_id: Uuid) -> AppResult<()> {
    let server = Self::get_server_by_id(db, server_id).await?;

    if server.owner_id != user_id {
      return Err(AppError::Unauthorized(
        "Only the server owner can delete the server".to_string(),
      ));
    }

    sqlx::query("DELETE FROM servers WHERE id = $1")
      .bind(server_id)
      .execute(db)
      .await?;

    Ok(())
  }

  pub async fn update_server(
    db: &PgPool,
    server_id: Uuid,
    user_id: Uuid,
    req: UpdateServerRequest,
  ) -> AppResult<Server> {
    let server = Self::get_server_by_id(db, server_id).await?;

    if server.owner_id != user_id {
      return Err(AppError::Unauthorized(
        "Only the server owner can update the server".to_string(),
      ));
    }

    if let Some(main_channel_id) = req.main_channel_id {
      let channel_server_id: Option<Uuid> =
        sqlx::query_scalar("SELECT server_id FROM channels WHERE id = $1")
          .bind(main_channel_id)
          .fetch_optional(db)
          .await?;

      if channel_server_id != Some(server_id) {
        return Err(AppError::BadRequest(
          "Channel does not belong to this server".to_string(),
        ));
      }
    }

    let server = sqlx::query_as::<_, Server>(
      r#"
      UPDATE servers
      SET
        name = COALESCE($1, name),
        main_channel_id = COALESCE($2, main_channel_id),
        updated_at = NOW()
      WHERE id = $3
      RETURNING id, name, owner_id, main_channel_id, created_at, updated_at
      "#,
    )
    .bind(req.name)
    .bind(req.main_channel_id)
    .bind(server_id)
    .fetch_one(db)
    .await?;

    Ok(server)
  }
}
