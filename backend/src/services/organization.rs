// backend/src/services/organization.rs
use crate::models::{
  BatchUpdateServerPositionsRequest, CreateFolderRequest, FolderResponse, OrganizedServersResponse,
  ServerFolder, ServerOrganization, ServerResponse, UpdateFolderRequest,
  UpdateServerOrganizationRequest,
};
use crate::services::ServerService;
use crate::utils::{AppError, AppResult};
use sqlx::PgPool;
use uuid::Uuid;

pub struct OrganizationService;

impl OrganizationService {
  // Folder management
  pub async fn create_folder(
    db: &PgPool,
    user_id: Uuid,
    req: CreateFolderRequest,
  ) -> AppResult<ServerFolder> {
    if req.name.trim().is_empty() {
      return Err(AppError::ValidationError(
        "Folder name cannot be empty".to_string(),
      ));
    }

    if let Some(ref color) = req.color {
      if !color.starts_with('#') || color.len() != 7 {
        return Err(AppError::ValidationError(
          "Color must be a hex code (e.g., #FF5733)".to_string(),
        ));
      }
    }

    let max_position: Option<i32> =
      sqlx::query_scalar("SELECT MAX(position) FROM server_folders WHERE user_id = $1")
        .bind(user_id)
        .fetch_one(db)
        .await?;

    let position = max_position.unwrap_or(-1) + 1;

    let folder = sqlx::query_as::<_, ServerFolder>(
      r#"
      INSERT INTO server_folders (user_id, name, color, position)
      VALUES ($1, $2, $3, $4)
      RETURNING id, user_id, name, color, position, created_at, updated_at
      "#,
    )
    .bind(user_id)
    .bind(&req.name)
    .bind(&req.color)
    .bind(position)
    .fetch_one(db)
    .await?;

    Ok(folder)
  }

  pub async fn update_folder(
    db: &PgPool,
    folder_id: Uuid,
    user_id: Uuid,
    req: UpdateFolderRequest,
  ) -> AppResult<ServerFolder> {
    let folder = Self::get_folder_by_id(db, folder_id).await?;

    if folder.user_id != user_id {
      return Err(AppError::Unauthorized(
        "You can only update your own folders".to_string(),
      ));
    }

    if let Some(ref color) = req.color {
      if !color.starts_with('#') || color.len() != 7 {
        return Err(AppError::ValidationError(
          "Color must be a hex code (e.g., #FF5733)".to_string(),
        ));
      }
    }

    let folder = sqlx::query_as::<_, ServerFolder>(
      r#"
      UPDATE server_folders
      SET 
        name = COALESCE($1, name),
        color = COALESCE($2, color),
        position = COALESCE($3, position),
        updated_at = NOW()
      WHERE id = $4
      RETURNING id, user_id, name, color, position, created_at, updated_at
      "#,
    )
    .bind(&req.name)
    .bind(&req.color)
    .bind(req.position)
    .bind(folder_id)
    .fetch_one(db)
    .await?;

    Ok(folder)
  }

  pub async fn delete_folder(db: &PgPool, folder_id: Uuid, user_id: Uuid) -> AppResult<()> {
    let folder = Self::get_folder_by_id(db, folder_id).await?;

    if folder.user_id != user_id {
      return Err(AppError::Unauthorized(
        "You can only delete your own folders".to_string(),
      ));
    }

    // Remove folder from servers (sets folder_id to NULL)
    sqlx::query("UPDATE server_organization SET folder_id = NULL WHERE folder_id = $1")
      .bind(folder_id)
      .execute(db)
      .await?;

    sqlx::query("DELETE FROM server_folders WHERE id = $1")
      .bind(folder_id)
      .execute(db)
      .await?;

    Ok(())
  }

  pub async fn get_folder_by_id(db: &PgPool, folder_id: Uuid) -> AppResult<ServerFolder> {
    let folder = sqlx::query_as::<_, ServerFolder>(
      r#"
      SELECT id, user_id, name, color, position, created_at, updated_at
      FROM server_folders
      WHERE id = $1
      "#,
    )
    .bind(folder_id)
    .fetch_optional(db)
    .await?
    .ok_or_else(|| AppError::NotFound("Folder not found".to_string()))?;

    Ok(folder)
  }

  // Server organization
  pub async fn update_server_organization(
    db: &PgPool,
    server_id: Uuid,
    user_id: Uuid,
    req: UpdateServerOrganizationRequest,
  ) -> AppResult<ServerOrganization> {
    // Verify user is a member of the server
    if !ServerService::is_member(db, server_id, user_id).await? {
      return Err(AppError::Unauthorized(
        "You must be a member of the server".to_string(),
      ));
    }

    // If folder_id is provided, verify it belongs to the user
    if let Some(folder_id) = req.folder_id {
      let folder = Self::get_folder_by_id(db, folder_id).await?;
      if folder.user_id != user_id {
        return Err(AppError::Unauthorized(
          "You can only add servers to your own folders".to_string(),
        ));
      }
    }

    let org = sqlx::query_as::<_, ServerOrganization>(
      r#"
      UPDATE server_organization
      SET 
        folder_id = COALESCE($1, folder_id),
        position = COALESCE($2, position),
        is_pinned = COALESCE($3, is_pinned),
        updated_at = NOW()
      WHERE user_id = $4 AND server_id = $5
      RETURNING user_id, server_id, folder_id, position, is_pinned, created_at, updated_at
      "#,
    )
    .bind(req.folder_id)
    .bind(req.position)
    .bind(req.is_pinned)
    .bind(user_id)
    .bind(server_id)
    .fetch_one(db)
    .await?;

    Ok(org)
  }

  pub async fn batch_update_server_positions(
    db: &PgPool,
    user_id: Uuid,
    req: BatchUpdateServerPositionsRequest,
  ) -> AppResult<()> {
    let mut tx = db.begin().await?;

    for update in req.updates {
      // Verify user is a member
      if !ServerService::is_member(&mut *tx, update.server_id, user_id).await? {
        continue;
      }

      // If folder_id is provided, verify it belongs to the user
      if let Some(folder_id) = update.folder_id {
        let folder_user_id: Option<Uuid> =
          sqlx::query_scalar("SELECT user_id FROM server_folders WHERE id = $1")
            .bind(folder_id)
            .fetch_optional(&mut *tx)
            .await?;

        if folder_user_id != Some(user_id) {
          continue;
        }
      }

      sqlx::query(
        r#"
        UPDATE server_organization
        SET 
          position = $1,
          folder_id = $2,
          updated_at = NOW()
        WHERE user_id = $3 AND server_id = $4
        "#,
      )
      .bind(update.position)
      .bind(update.folder_id)
      .bind(user_id)
      .bind(update.server_id)
      .execute(&mut *tx)
      .await?;
    }

    tx.commit().await?;

    Ok(())
  }

  pub async fn get_organized_servers(
    db: &PgPool,
    user_id: Uuid,
  ) -> AppResult<OrganizedServersResponse> {
    // Get all folders
    let folders = sqlx::query_as::<_, ServerFolder>(
      r#"
      SELECT id, user_id, name, color, position, created_at, updated_at
      FROM server_folders
      WHERE user_id = $1
      ORDER BY position ASC
      "#,
    )
    .bind(user_id)
    .fetch_all(db)
    .await?;

    let mut folder_responses = Vec::new();

    for folder in folders {
      // Get servers in this folder
      let servers = sqlx::query_as::<_, (Uuid, String, Uuid, Option<Uuid>, i32)>(
        r#"
        SELECT s.id, s.name, s.owner_id, s.main_channel_id, so.position
        FROM servers s
        INNER JOIN server_organization so ON s.id = so.server_id
        WHERE so.user_id = $1 AND so.folder_id = $2
        ORDER BY so.position ASC
        "#,
      )
      .bind(user_id)
      .bind(folder.id)
      .fetch_all(db)
      .await?;

      let server_responses: Vec<ServerResponse> = servers
        .into_iter()
        .map(|(id, name, owner_id, main_channel_id, _)| ServerResponse {
          id,
          name,
          owner_id,
          main_channel_id,
          is_owner: owner_id == user_id,
          created_at: chrono::Utc::now(), // We could fetch this if needed
        })
        .collect();

      folder_responses.push(FolderResponse {
        id: folder.id,
        name: folder.name,
        color: folder.color,
        position: folder.position,
        servers: server_responses,
        created_at: folder.created_at,
      });
    }

    // Get ungrouped servers
    let ungrouped = sqlx::query_as::<_, (Uuid, String, Uuid, Option<Uuid>)>(
      r#"
      SELECT s.id, s.name, s.owner_id, s.main_channel_id
      FROM servers s
      INNER JOIN server_organization so ON s.id = so.server_id
      WHERE so.user_id = $1 AND so.folder_id IS NULL
      ORDER BY so.position ASC
      "#,
    )
    .bind(user_id)
    .fetch_all(db)
    .await?;

    let ungrouped_servers: Vec<ServerResponse> = ungrouped
      .into_iter()
      .map(|(id, name, owner_id, main_channel_id)| ServerResponse {
        id,
        name,
        owner_id,
        is_owner: owner_id == user_id,
        main_channel_id,
        created_at: chrono::Utc::now(),
      })
      .collect();

    Ok(OrganizedServersResponse {
      folders: folder_responses,
      ungrouped_servers,
    })
  }
}
