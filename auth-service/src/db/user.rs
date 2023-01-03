use std::sync::Arc;

use crate::config::crypto::CryptoService;
use crate::models::{NewUser, User};
use color_eyre::Result;
use sqlx::PgPool;

pub struct UserRepository {
  pool: Arc<PgPool>,
}

impl UserRepository {
  pub fn new(pool: Arc<PgPool>) -> Self {
    Self { pool }
  }

  pub async fn create(&self, new_user: NewUser, crypto_service: &CryptoService) -> Result<User> {
    let password_hash = crypto_service.hash_password(new_user.password).await?;
    let user = sqlx::query_as::<_, User>(
      "insert into users (username, email, full_name, bio, image, password_hash) values ($1, $2, $3, $4, $5, $6) returning *",
    )
    .bind(new_user.username)
    .bind(new_user.email)
    .bind(new_user.full_name)
    .bind(new_user.bio)
    .bind(new_user.image)
    .bind(password_hash)
    .fetch_one(&*self.pool)
    .await?;
    Ok(user)
  }
}
