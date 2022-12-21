use crate::domain::Asker;
use spinners::{Spinner, Spinners};
use tokio::io::{stdout, AsyncBufReadExt, AsyncWriteExt};

pub async fn run_cli(asker: impl Asker) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  println!("{esc}c", esc = 27 as char);

  let mut stdin = tokio::io::BufReader::new(tokio::io::stdin());

  loop {
    print!("> ");
    stdout().flush().await?;
    let mut user_text = String::new();

    stdin.read_line(&mut user_text).await?;

    println!();

    let mut sp = Spinner::new(Spinners::Dots4, "\t\tOpenAI is thinking...".into());
    let text = user_text.trim();
    let answer = asker.ask(text).await?;

    sp.stop();

    println!();
    println!("{answer}");
  }
}

#[cfg(test)]
mod test {}
