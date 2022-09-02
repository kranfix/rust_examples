mod config;
mod handlers;
mod models;

use crate::config::Config;
use actix_web::{middleware::Logger, web, App, HttpServer};
use color_eyre::Result;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
  let config = Config::from_env().expect("Server configuration");

  let pool = config.db_pool().await.expect("Database configuration");
  let crypto_service = config.crypto_service();

  info!("Starting server at http://{}:{}", config.host, config.port);
  HttpServer::new(move || {
    App::new() //
      .wrap(Logger::default())
      .app_data(pool.clone())
      .app_data(crypto_service.clone())
      .route("/", web::get().to(|| async { "Hello, there!" }))
      .service(
        web::scope("/hello") //
          .configure(crate::handlers::hello_scoped_config),
      )
    //.service(crate::handlers::hello)
  })
  .bind(format!("{}:{}", config.host, config.port))?
  .run()
  .await?;

  Ok(())
}
