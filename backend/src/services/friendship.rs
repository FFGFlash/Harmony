use sqlx::PgPool;
use uuid::Uuid;

use crate::{
  models::{Friendship, Profile, User},
  utils::{AppError, AppResult},
};

pub struct FriendshipService;

impl FriendshipService {
  pub async fn get_friends(db: &PgPool, user_id: Uuid) -> AppResult<Vec<Profile>> {
    let friends = sqlx::query_as::<_, Profile>(
      r#"
      SELECT u.id, u.username, u.created_at
      FROM friendships f
      JOIN users u
        ON u.id = CASE
          WHEN f.user_low = $1 THEN f.user_high
          ELSE f.user_low
        END
      WHERE f.status = 'accepted'
        AND (f.user_low = $1 OR f.user_high = $1)
      "#,
    )
    .bind(user_id)
    .fetch_all(db)
    .await?;
    Ok(friends)
  }

  pub async fn get_incoming_requests(db: &PgPool, user_id: Uuid) -> AppResult<Vec<Profile>> {
    let requests = sqlx::query_as::<_, Profile>(
      r#"
      SELECT u.id, u.username, u.created_at
      FROM friendships f
      JOIN users u
        ON u.id = CASE
          WHEN f.user_low = $1 THEN f.user_high
          ELSE f.user_low
        END
      WHERE f.status = 'pending'
        AND (f.user_low = $1 OR f.user_high = $1)
        AND f.sender_id <> $1
      "#,
    )
    .bind(user_id)
    .fetch_all(db)
    .await?;
    Ok(requests)
  }

  pub async fn get_outgoing_requests(db: &PgPool, user_id: Uuid) -> AppResult<Vec<Profile>> {
    let requests = sqlx::query_as::<_, Profile>(
      r#"
      SELECT u.id, u.username, u.created_at
      FROM friendships f
      JOIN users u
        ON u.id = CASE
          WHEN f.user_low = $1 THEN f.user_high
          ELSE f.user_low
        END
      WHERE f.status = 'pending'
        AND f.sender_id = $1
      "#,
    )
    .bind(user_id)
    .fetch_all(db)
    .await?;
    Ok(requests)
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

    if FriendshipService::user_is_blocked_by(db, user_id, other_id).await? {
      return Err(AppError::BadRequest(
        "Unable to send friend request, blocked".to_string(),
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

  pub async fn _block_user(db: &PgPool, user_id: Uuid, other_id: Uuid) -> AppResult<Friendship> {
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
}
