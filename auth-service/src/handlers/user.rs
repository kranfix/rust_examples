use crate::config::crypto::CryptoService;
use crate::db::UserRepository;
use crate::models::NewUser;
use actix_web::web;
use actix_web::{get, post, web::ServiceConfig, Responder};
use sqlx::PgPool;

#[post("")]
async fn create(
  pool: web::Data<PgPool>,
  crypto_service: web::Data<CryptoService>,
) -> impl Responder {
  let new_user = NewUser {
    username: "kranfix".to_string(),
    email: "frank@kranfix.dev".to_string(),
    full_name: "Frank Moreno".to_string(),
    bio: "I'm engineer".to_string(),
    image: "https://en.gravatar.com/userimage/33078272/93066c7beee8b7fe33c31eb37efd1527.jpg"
      .to_string(),
    password: "123456".to_string(),
  };
  let repo = UserRepository::new((*pool).clone());

  match repo.create(new_user, &crypto_service).await {
    Ok(user) => web::Json(user),
    Err(_) => todo!(),
  }
}

#[get("")]
async fn list_users() -> impl Responder {
  "Hello"
}

pub fn user_scoped_config(cfg: &mut ServiceConfig) {
  cfg //
    .service(create)
    .service(list_users);
}
