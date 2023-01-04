use actix_web::{
  get,
  web::{self, ServiceConfig},
  Responder,
};

#[get("")]
async fn hello() -> impl Responder {
  "Hello, there!"
}

#[get("/{name}")]
async fn hello_name(name: web::Path<String>) -> impl Responder {
  format!("Hello, {name}")
}

pub fn hello_scoped_config(cfg: &mut ServiceConfig) {
  cfg //
    .service(hello)
    .service(hello_name);
}
