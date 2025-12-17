// backend/src/models/pagination.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
  pub data: Vec<T>,
  pub total: i64,
  pub limit: i64,
  pub offset: i64,
  pub has_more: bool,
}

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
  #[serde(default = "default_limit")]
  pub limit: i64,
  #[serde(default)]
  pub offset: i64,
}

fn default_limit() -> i64 {
  50
}

impl PaginationParams {
  pub fn validate(&self) -> Result<(), String> {
    if self.limit < 1 {
      return Err("Limit must be at least 1".to_string());
    }
    if self.limit > 100 {
      return Err("Limit cannot exceed 100".to_string());
    }
    if self.offset < 0 {
      return Err("Offset cannot be negative".to_string());
    }
    Ok(())
  }

  pub fn sanitize(&self) -> Self {
    Self {
      limit: self.limit.clamp(1, 100),
      offset: self.offset.max(0),
    }
  }
}
