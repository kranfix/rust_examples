use ai_asker_cli::cli::run_cli;
use ai_asker_cli::domain::FakeAsker;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  let fake = FakeAsker::new(2000);
  run_cli(fake).await
}
