use actix_web::{
  get,
  web::{self, ServiceConfig},
  Responder,
};

#[get("")]
async fn hello() -> impl Responder {
  format!("Hello, there!")
}

#[get("/{name}")]
async fn hello_name(name: web::Path<String>) -> impl Responder {
  format!("Hello, {name}")
}

pub fn hello_scoped_config(cfg: &mut ServiceConfig) {
  cfg
    .service(crate::handlers::hello)
    .service(crate::handlers::hello_name);
}
