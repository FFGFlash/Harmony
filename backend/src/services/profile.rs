use crate::models::{FullProfile, Profile, UpdateProfileRequest};
use crate::utils::{AppError, AppResult};
use sqlx::PgPool;
use uuid::Uuid;

pub struct ProfileService;

impl ProfileService {
  pub async fn get_full_profile(db: &PgPool, user_id: Uuid) -> AppResult<FullProfile> {
    let profile = sqlx::query_as::<_, FullProfile>(
      r#"
      SELECT
        u.id,
        u.username,
        p.display_name,
        p.bio,
        p.avatar_url,
        p.banner_url,
        p.status,
        p.custom_status,
        p.status_emoji,
        p.show_online_status,
        u.created_at
      FROM users u
      LEFT JOIN profiles p ON u.id = p.user_id
      WHERE u.id = $1
      "#,
    )
    .bind(user_id)
    .fetch_optional(db)
    .await?
    .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    Ok(profile)
  }

  pub async fn get_full_profile_by_username(db: &PgPool, username: &str) -> AppResult<FullProfile> {
    let profile = sqlx::query_as::<_, FullProfile>(
      r#"
      SELECT
        u.id,
        u.username,
        p.display_name,
        p.bio,
        p.avatar_url,
        p.banner_url,
        p.status,
        p.custom_status,
        p.status_emoji,
        p.show_online_status,
        u.created_at
      FROM users u
      LEFT JOIN profiles p ON u.id = p.user_id
      WHERE u.username = $1
      "#,
    )
    .bind(username)
    .fetch_optional(db)
    .await?
    .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    Ok(profile)
  }

  pub async fn update_profile(
    db: &PgPool,
    user_id: Uuid,
    req: UpdateProfileRequest,
  ) -> AppResult<Profile> {
    if let Some(ref display_name) = req.display_name {
      if display_name.len() > 100 {
        return Err(AppError::ValidationError(
          "Display name cannot exceed 100 characters".to_string(),
        ));
      }
    }

    if let Some(ref bio) = req.bio {
      if bio.len() > 500 {
        return Err(AppError::ValidationError(
          "Bio cannot exceed 500 characters".to_string(),
        ));
      }
    }

    let profile = sqlx::query_as::<_, Profile>(
      r#"
      UPDATE profiles
      SET
        display_name = COALESCE($2, display_name),
        bio = COALESCE($3, bio),
        avatar_url = COALESCE($4, avatar_url),
        banner_url = COALESCE($5, banner_url),
        status = COALESCE($6, status),
        custom_status = COALESCE($7, custom_status),
        status_emoji = COALESCE($8, status_emoji),
        show_online_status = COALESCE($9, show_online_status),
        allow_dms = COALESCE($10, allow_dms),
        updated_at = NOW()
      WHERE user_id = $1
      RETURNING user_id, display_name, bio, avatar_url, banner_url,
        status, custom_status, status_emoji, show_online_status,
        allow_dms, created_at, updated_at
      "#,
    )
    .bind(user_id)
    .bind(&req.display_name)
    .bind(&req.bio)
    .bind(&req.avatar_url)
    .bind(&req.banner_url)
    .bind(&req.status)
    .bind(&req.custom_status)
    .bind(&req.status_emoji)
    .bind(&req.show_online_status)
    .bind(&req.allow_dms)
    .fetch_one(db)
    .await?;

    Ok(profile)
  }

  pub async fn get_profile(db: &PgPool, user_id: Uuid) -> AppResult<Profile> {
    let profile = sqlx::query_as::<_, Profile>(
      r#"
      SELECT
        user_id, display_name, bio, avatar_url, banner_url,
        status, custom_status, status_emoji, show_online_status,
        allow_dms, created_at, updated_at
      FROM profiles
      WHERE user_id = $1
      "#,
    )
    .bind(user_id)
    .fetch_optional(db)
    .await?
    .ok_or_else(|| AppError::NotFound("Profile not found".to_string()))?;

    Ok(profile)
  }
}
