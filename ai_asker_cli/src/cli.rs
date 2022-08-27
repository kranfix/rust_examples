use crate::domain::Asker;
use spinners::{Spinner, Spinners};
use std::io::{stdin, stdout, Write};

pub async fn run_cli(asker: impl Asker) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  println!("{esc}c", esc = 27 as char);

  loop {
    print!("> ");
    stdout().flush().unwrap();
    let mut user_text = String::new();

    stdin()
      .read_line(&mut user_text)
      .expect("Failed to read line");

    println!("");

    let mut sp = Spinner::new(Spinners::Dots4, "\t\tOpenAI is thinking...".into());
    let text = user_text.trim().into();
    let answer = asker.ask(&text).await?;

    sp.stop();

    println!("");
    println!("{answer}");
  }
}

#[cfg(test)]
mod test {}
