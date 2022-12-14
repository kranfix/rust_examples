use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct User {
  pub id: Uuid,
  pub username: String,
  pub email: String,
  #[serde(skip_serializing)]
  pub password_hash: String,
  pub full_name: Option<String>,
  pub bio: Option<String>,
  pub image: Option<String>,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Validate)]
pub struct NewUser {
  pub username: String,
  pub email: String,
  #[validate(length(min = 3))]
  pub full_name: String,
  pub bio: String,
  pub image: String,
  #[validate(length(min = 6))]
  pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProfile {
  pub full_name: Option<String>,
  pub bio: Option<String>,
  #[validate(url)]
  pub image: Option<String>,
}
