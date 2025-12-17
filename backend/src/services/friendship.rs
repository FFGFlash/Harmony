use sqlx::PgPool;
use uuid::Uuid;

use crate::{
  models::{Friendship, FullProfile, PaginatedResponse, Profile},
  utils::{AppError, AppResult},
};

pub struct FriendshipService;

impl FriendshipService {
  pub async fn get_friends(
    db: &PgPool,
    user_id: Uuid,
    limit: i64,
    offset: i64,
  ) -> AppResult<PaginatedResponse<FullProfile>> {
    // Get total count
    let total: i64 = sqlx::query_scalar(
      r#"
      SELECT COUNT(*)
      FROM friendships f
      WHERE f.status = 'accepted'
        AND (f.user_low = $1 OR f.user_high = $1)
      "#,
    )
    .bind(user_id)
    .fetch_one(db)
    .await?;

    // Get paginated data
    let friends = sqlx::query_as::<_, FullProfile>(
      r#"
      SELECT
        u.id, u.username, p.display_name, p.bio, p.avatar_url, p.banner_url,
        p.status, p.custom_status, p.status_emoji, p.show_online_status,
        p.created_at
      FROM friendships f
      JOIN users u
        ON u.id = CASE
          WHEN f.user_low = $1 THEN f.user_high
          ELSE f.user_low
        END
      JOIN profiles p ON p.user_id = u.id
      WHERE f.status = 'accepted'
        AND (f.user_low = $1 OR f.user_high = $1)
      ORDER BY u.username ASC
      LIMIT $2 OFFSET $3
      "#,
    )
    .bind(user_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(db)
    .await?;

    Ok(PaginatedResponse {
      data: friends,
      total,
      limit,
      offset,
      has_more: offset + limit < total,
    })
  }

  pub async fn user_is_blocked_by(
    db: &PgPool,
    sender_id: Uuid,
    recipient_id: Uuid,
  ) -> AppResult<bool> {
    let blocked: bool = sqlx::query_scalar(
      r#"
      SELECT EXISTS(
        SELECT 1 FROM friendships
        WHERE user_low = LEAST($1, $2)
          AND user_high = GREATEST($1, $2)
          AND status = 'blocked'
      )
      "#,
    )
    .bind(sender_id)
    .bind(recipient_id)
    .fetch_one(db)
    .await?;

    Ok(blocked)
  }

  pub async fn get_incoming_requests(
    db: &PgPool,
    user_id: Uuid,
    limit: i64,
    offset: i64,
  ) -> AppResult<PaginatedResponse<FullProfile>> {
    // Get total count
    let total: i64 = sqlx::query_scalar(
      r#"
      SELECT COUNT(*)
      FROM friendships f
      WHERE f.status = 'pending'
        AND (f.user_low = $1 OR f.user_high = $1)
        AND f.sender_id <> $1
      "#,
    )
    .bind(user_id)
    .fetch_one(db)
    .await?;

    // Get paginated data
    let requests = sqlx::query_as::<_, FullProfile>(
      r#"
      SELECT 
        u.id, u.username, p.display_name, p.bio, p.avatar_url, p.banner_url,
        p.status, p.custom_status, p.status_emoji, p.show_online_status,
        p.created_at
      FROM friendships f
      JOIN users u ON u.id = f.sender_id
      JOIN profiles p ON p.user_id = f.sender_id
      WHERE f.status = 'pending'
        AND (f.user_low = $1 OR f.user_high = $1)
        AND f.sender_id <> $1
      ORDER BY f.created_at DESC
      LIMIT $2 OFFSET $3
      "#,
    )
    .bind(user_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(db)
    .await?;

    Ok(PaginatedResponse {
      data: requests,
      total,
      limit,
      offset,
      has_more: offset + limit < total,
    })
  }

  pub async fn get_outgoing_requests(
    db: &PgPool,
    user_id: Uuid,
    limit: i64,
    offset: i64,
  ) -> AppResult<PaginatedResponse<FullProfile>> {
    // Get total count
    let total: i64 = sqlx::query_scalar(
      r#"
      SELECT COUNT(*)
      FROM friendships f
      WHERE f.status = 'pending'
        AND f.sender_id = $1
      "#,
    )
    .bind(user_id)
    .fetch_one(db)
    .await?;

    // Get paginated data
    let requests = sqlx::query_as::<_, FullProfile>(
      r#"
      SELECT
        u.id, u.username, p.display_name, p.bio, p.avatar_url, p.banner_url,
        p.status, p.custom_status, p.status_emoji, p.show_online_status,
        p.created_at
      FROM friendships f
      JOIN users u
        ON u.id = CASE
          WHEN f.user_low = $1 THEN f.user_high
          ELSE f.user_low
        END
      JOIN profiles p ON p.user_id = u.id
      WHERE f.status = 'pending'
        AND f.sender_id = $1
      ORDER BY f.created_at DESC
      LIMIT $2 OFFSET $3
      "#,
    )
    .bind(user_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(db)
    .await?;

    Ok(PaginatedResponse {
      data: requests,
      total,
      limit,
      offset,
      has_more: offset + limit < total,
    })
  }

  pub async fn send_or_accept_request(
    db: &PgPool,
    user_id: Uuid,
    other_id: Uuid,
  ) -> AppResult<Friendship> {
    if user_id == other_id {
      return Err(AppError::BadRequest(
        "You cannot send a friend request to yourself".to_string(),
      ));
    }

    if Self::user_is_blocked_by(db, user_id, other_id).await? {
      return Err(AppError::BadRequest(
        "Unable to send friend request".to_string(),
      ));
    }

    let friendship = sqlx::query_as::<_, Friendship>(
      r#"
      INSERT INTO friendships (user_low, user_high, sender_id, status)
      VALUES (
        LEAST($1, $2),
        GREATEST($1, $2),
        $1,
        'pending'
      )
      ON CONFLICT (user_low, user_high)
      DO UPDATE
      SET status = CASE
          WHEN friendships.status = 'pending'
            AND friendships.sender_id <> EXCLUDED.sender_id
          THEN 'accepted'
          WHEN friendships.status = 'rejected'
          THEN 'pending'
          ELSE friendships.status
        END,
        sender_id = CASE
          WHEN friendships.status = 'rejected'
            AND friendships.sender_id <> EXCLUDED.sender_id
          THEN EXCLUDED.sender_id
          ELSE friendships.sender_id
        END,
        updated_at = CASE
          WHEN (
              friendships.status = 'pending'
              AND friendships.sender_id <> EXCLUDED.sender_id
            )
            OR friendships.status = 'rejected'
          THEN now()
          ELSE friendships.updated_at
        END
      RETURNING user_low, user_high, sender_id, status, created_at, updated_at
      "#,
    )
    .bind(user_id)
    .bind(other_id)
    .fetch_one(db)
    .await?;

    Ok(friendship)
  }

  pub async fn reject_request(db: &PgPool, user_id: Uuid, other_id: Uuid) -> AppResult<()> {
    let result = sqlx::query(
      r#"
      UPDATE friendships
      SET status = 'rejected',
          updated_at = now()
      WHERE user_low = LEAST($1, $2)
        AND user_high = GREATEST($1, $2)
        AND status = 'pending'
        AND sender_id = $2
      "#,
    )
    .bind(user_id)
    .bind(other_id)
    .execute(db)
    .await?;

    if result.rows_affected() == 0 {
      return Err(AppError::NotFound("Friend request not found".to_string()));
    }

    Ok(())
  }

  pub async fn remove_friend(db: &PgPool, user_id: Uuid, other_id: Uuid) -> AppResult<()> {
    let result = sqlx::query(
      r#"
      DELETE FROM friendships
      WHERE user_low = LEAST($1, $2)
        AND user_high = GREATEST($1, $2)
        AND (user_low = $1 OR user_high = $1)
      "#,
    )
    .bind(user_id)
    .bind(other_id)
    .execute(db)
    .await?;

    if result.rows_affected() == 0 {
      return Err(AppError::NotFound("Friendship not found".to_string()));
    }

    Ok(())
  }

  pub async fn block_user(db: &PgPool, user_id: Uuid, other_id: Uuid) -> AppResult<Friendship> {
    if user_id == other_id {
      return Err(AppError::BadRequest(
        "You cannot block yourself".to_string(),
      ));
    }

    let friendship = sqlx::query_as::<_, Friendship>(
      r#"
      INSERT INTO friendships (user_low, user_high, sender_id, status)
      VALUES (
        LEAST($1, $2),
        GREATEST($1, $2),
        $1,
        'blocked'
      )
      ON CONFLICT (user_low, user_high)
      DO UPDATE SET
        status = 'blocked',
        sender_id = $1,
        updated_at = now()
      RETURNING user_low, user_high, sender_id, status, created_at, updated_at
      "#,
    )
    .bind(user_id)
    .bind(other_id)
    .fetch_one(db)
    .await?;

    Ok(friendship)
  }

  pub async fn unblock_user(db: &PgPool, user_id: Uuid, other_id: Uuid) -> AppResult<()> {
    let result = sqlx::query(
      r#"
      DELETE FROM friendships
      WHERE user_low = LEAST($1, $2)
        AND user_high = GREATEST($1, $2)
        AND status = 'blocked'
        AND sender_id = $1
      "#,
    )
    .bind(user_id)
    .bind(other_id)
    .execute(db)
    .await?;

    if result.rows_affected() == 0 {
      return Err(AppError::NotFound("Block not found".to_string()));
    }

    Ok(())
  }

  pub async fn get_blocked_users(
    db: &PgPool,
    user_id: Uuid,
    limit: i64,
    offset: i64,
  ) -> AppResult<PaginatedResponse<Profile>> {
    // Get total count
    let total: i64 = sqlx::query_scalar(
      r#"
      SELECT COUNT(*)
      FROM friendships f
      WHERE f.status = 'blocked'
        AND f.sender_id = $1
      "#,
    )
    .bind(user_id)
    .fetch_one(db)
    .await?;

    // Get paginated data
    let blocked = sqlx::query_as::<_, Profile>(
      r#"
      SELECT
        p.user_id, p.display_name, p.bio, p.avatar_url, p.banner_url,
        p.status, p.custom_status, p.status_emoji, p.show_online_status,
        p.allow_dms, p.created_at, p.updated_at
      FROM friendships f
      JOIN profiles p
        ON p.user_id = CASE
          WHEN f.user_low = $1 THEN f.user_high
          ELSE f.user_low
        END
      WHERE f.status = 'blocked'
        AND f.sender_id = $1
      ORDER BY f.created_at DESC
      LIMIT $2 OFFSET $3
      "#,
    )
    .bind(user_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(db)
    .await?;

    Ok(PaginatedResponse {
      data: blocked,
      total,
      limit,
      offset,
      has_more: offset + limit < total,
    })
  }

  pub async fn search_users_by_username(
    db: &PgPool,
    username: &str,
    limit: i64,
    offset: i64,
  ) -> AppResult<PaginatedResponse<FullProfile>> {
    // Get total count
    let total: i64 = sqlx::query_scalar(
      r#"
      SELECT COUNT(*)
      FROM users
      WHERE username ILIKE $1
      "#,
    )
    .bind(format!("%{}%", username))
    .fetch_one(db)
    .await?;

    // Get paginated data
    let users = sqlx::query_as::<_, FullProfile>(
      r#"
      SELECT
        u.id, u.username, p.display_name, p.bio, p.avatar_url, p.banner_url,
        p.status, p.custom_status, p.status_emoji, p.show_online_status,
        p.allow_dms, p.created_at, p.updated_at
      FROM users u
      LEFT JOIN profiles p
      ON u.id = p.user_id
      WHERE u.username ILIKE $1
      ORDER BY u.username ASC
      LIMIT $2 OFFSET $3
      "#,
    )
    .bind(format!("%{}%", username))
    .bind(limit)
    .bind(offset)
    .fetch_all(db)
    .await?;

    Ok(PaginatedResponse {
      data: users,
      total,
      limit,
      offset,
      has_more: offset + limit < total,
    })
  }

  pub async fn get_user_by_username(db: &PgPool, username: &str) -> AppResult<Profile> {
    let user = sqlx::query_as::<_, Profile>(
      r#"
      SELECT
        p.user_id, p.display_name, p.bio, p.avatar_url, p.banner_url,
        p.status, p.custom_status, p.status_emoji, p.show_online_status,
        p.allow_dms, p.created_at, p.updated_at
      FROM users u
      LEFT JOIN profiles p
      ON u.id = p.user_id
      WHERE username = $1
      "#,
    )
    .bind(username)
    .fetch_optional(db)
    .await?
    .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    Ok(user)
  }

  pub async fn get_user_profile(db: &PgPool, user_id: Uuid) -> AppResult<Profile> {
    let user = sqlx::query_as::<_, Profile>(
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
    .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    Ok(user)
  }
}
