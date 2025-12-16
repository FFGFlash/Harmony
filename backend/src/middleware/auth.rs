use crate::services::AuthService;
use crate::utils::{AppError, AppResult};
use axum::{extract::Request, http::header, middleware::Next, response::Response};
use uuid::Uuid;

#[derive(Clone)]
pub struct CurrentUser {
  pub id: Uuid,
}

pub async fn auth_middleware(mut req: Request, next: Next) -> AppResult<Response> {
  let auth_header = req
    .headers()
    .get(header::AUTHORIZATION)
    .and_then(|header| header.to_str().ok());

  let token = auth_header
    .and_then(|header| {
      if header.starts_with("Bearer ") {
        Some(header.trim_start_matches("Bearer "))
      } else {
        None
      }
    })
    .ok_or_else(|| AppError::Unauthorized("Missing or invalid authorization header".to_string()))?;

  let claims = AuthService::verify_token(token)?;

  let user_id = Uuid::parse_str(&claims.sub)
    .map_err(|_| AppError::Unauthorized("Invalid user ID in token".to_string()))?;

  req.extensions_mut().insert(CurrentUser { id: user_id });

  Ok(next.run(req).await)
}

#[allow(dead_code)]
pub trait RequestExt {
  fn current_user(&self) -> Option<&CurrentUser>;
}

impl RequestExt for Request {
  fn current_user(&self) -> Option<&CurrentUser> {
    self.extensions().get::<CurrentUser>()
  }
}
