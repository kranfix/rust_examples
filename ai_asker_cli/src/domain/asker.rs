use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

use async_trait::async_trait;

#[async_trait]
pub trait Asker {
  async fn ask(
    &self,
    question: &str,
  ) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;
}

pub struct FakeAsker {
  len: u8,
  millis: u64,
  counter: Arc<Mutex<u8>>,
}

impl FakeAsker {
  pub fn new(millis: u64) -> FakeAsker {
    FakeAsker {
      len: 4,
      millis,
      counter: Arc::new(Mutex::new(0)),
    }
  }
}

#[async_trait]
impl Asker for FakeAsker {
  async fn ask(&self, question: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    sleep(Duration::from_millis(self.millis)).await;
    if question == "fail" {
      Err("failed")?;
    }

    let mut count = self.counter.lock().await;

    let idx = (*count) as usize;
    let answer = 
      format!("Answer {idx}: Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.");

    *count = if *count < self.len-1 { *count + 1 } else { 0 };

    Ok(answer)
  }
}

#[cfg(test)]
mod test {
  use crate::aw;
  use super::{Asker, FakeAsker};
  

  #[test]
  fn test_faker() {
    let fake = FakeAsker::new(100);

    for i in 0..fake.len {
      let answer = aw!(fake.ask("question")).unwrap();
      assert_eq!(answer[..8], format!("Answer {i}"));
    }

    let answer = aw!(fake.ask("question")).unwrap();
    assert_eq!(answer[..8], format!("Answer 0"));

    let err = aw!(fake.ask("fail")).unwrap_err();
    let err = format!("{err}");
    assert_eq!(err, "failed");
  }
}
