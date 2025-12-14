use crate::models::{CreateUserRequest, LoginRequest, User};
use crate::utils::{AppError, AppResult};
use argon2::{
  Argon2,
  password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::env;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
  pub sub: String, // user id
  pub exp: i64,    // expiration time
  pub iat: i64,    // issued at
}

pub struct AuthService;

impl AuthService {
  pub fn hash_password(password: &str) -> AppResult<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
      .hash_password(password.as_bytes(), &salt)
      .map_err(|e| AppError::InternalServerError(format!("Failed to hash password: {}", e)))
      .map(|hash| hash.to_string())
  }

  pub fn verify_password(password: &str, password_hash: &str) -> AppResult<bool> {
    let parsed_hash = PasswordHash::new(password_hash)
      .map_err(|e| AppError::InternalServerError(format!("Invalid password hash: {}", e)))?;

    Ok(
      Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok(),
    )
  }

  pub fn generate_token(user_id: Uuid) -> AppResult<String> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let expiration = env::var("JWT_EXPIRATION")
      .unwrap_or_else(|_| "86400".to_string())
      .parse::<i64>()
      .unwrap_or(86400);

    let now = Utc::now();
    let exp = (now + Duration::seconds(expiration)).timestamp();

    let claims = Claims {
      sub: user_id.to_string(),
      exp,
      iat: now.timestamp(),
    };

    encode(
      &Header::default(),
      &claims,
      &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::InternalServerError(format!("Failed to generate token: {}", e)))
  }

  pub fn verify_token(token: &str) -> AppResult<Claims> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    decode::<Claims>(
      token,
      &DecodingKey::from_secret(secret.as_bytes()),
      &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| AppError::InternalServerError(format!("Invalid token: {}", e)))
  }

  pub async fn create_user(db: &PgPool, req: CreateUserRequest) -> AppResult<User> {
    let username = req.username.trim().to_lowercase();
    let email = req.email.trim();
    let password = req.password.trim();

    if username.is_empty() || email.is_empty() || password.is_empty() {
      return Err(AppError::ValidationError(
        "All fields are required".to_string(),
      ));
    }

    if password.len() < 8 {
      return Err(AppError::ValidationError(
        "Password must be at least 8 characters".to_string(),
      ));
    }

    let password_hash = Self::hash_password(&password)?;

    let user = sqlx::query_as::<_, User>(
      r#"
      INSERT INTO users (username, email, password_hash)
      VALUES ($1, $2, $3)
      RETURNING id, username, email, password_hash, created_at, updated_at
      "#,
    )
    .bind(&username)
    .bind(&email)
    .bind(&password_hash)
    .fetch_one(db)
    .await
    .map_err(|e| match e {
      sqlx::Error::Database(database_error) if database_error.is_unique_violation() => {
        AppError::BadRequest("Username or email already exists".to_string())
      }
      _ => AppError::from(e),
    })?;

    Ok(user)
  }

  pub async fn authenticate_user(db: &PgPool, req: LoginRequest) -> AppResult<User> {
    let mut user: Option<User> = None;

    if let Some(email) = req.email {
      user = Some(
        sqlx::query_as::<_, User>(
          r#"
        SELECT id, username, email, password_hash, created_at, updated_at
        FROM users
        WHERE email = $1
        "#,
        )
        .bind(&email)
        .fetch_optional(db)
        .await?
        .ok_or_else(|| AppError::Unauthorized("Invalid login information".to_string()))?,
      );
    } else if let Some(username) = req.username {
      user = Some(
        sqlx::query_as::<_, User>(
          r#"
        SELECT id, username, email, password_hash, created_at, updated_at
        FROM users
        WHERE username = $1
        "#,
        )
        .bind(&username)
        .fetch_optional(db)
        .await?
        .ok_or_else(|| AppError::Unauthorized("Invalid login information".to_string()))?,
      );
    }

    let user =
      user.ok_or_else(|| AppError::Unauthorized("Invalid login information".to_string()))?;

    if !Self::verify_password(&req.password, &user.password_hash)? {
      return Err(AppError::Unauthorized(
        "Invalid login information".to_string(),
      ));
    }

    Ok(user)
  }
}
