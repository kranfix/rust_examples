use ai_asker_cli::cli::run_cli;
use ai_asker_cli::openai_client::OAIClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  let oai_token = env::var("OAI_TOKEN")?;
  let client = OAIClient::new(oai_token);
  run_cli(client).await
}
