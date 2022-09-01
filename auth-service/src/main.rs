mod config;
mod handlers;

use crate::config::Config;
use actix_web::{middleware::Logger, web, App, HttpServer};
use color_eyre::Result;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
  let config = Config::from_env().expect("Server configuration");

  info!("Starting server at http://{}:{}", config.host, config.port);
  HttpServer::new(move || {
    App::new() //
      .wrap(Logger::default())
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
