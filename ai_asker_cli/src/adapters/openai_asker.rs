use crate::{domain::Asker, openai_client::OAIClientApi};
use async_trait::async_trait;

#[async_trait]
impl<T: OAIClientApi> Asker for T {
  async fn ask(
    &self,
    question: &String,
  ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let json = self.request_completitions(question).await?;
    Ok(json.choices[0].text.to_owned())
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::openai_client::*;

  fn completitions_mock(
    question: &String,
  ) -> Result<OAIResponse, Box<dyn std::error::Error + Send + Sync>> {
    if question == "fail" {
      Err("failed")?
    }
    Ok(OAIResponse {
      choices: vec![OAIChoices {
        text: "This is an answer".into(),
      }],
    })
  }

  #[tokio::test]
  async fn client_test() {
    let mut mock = MockOAIClientApi::new();

    mock
      .expect_request_completitions()
      .times(2)
      .returning(|question| {
        let res = completitions_mock(question);
        Box::pin(async { res })
      });

    let answer = mock.ask(&"question".into()).await.unwrap();
    assert_eq!(answer, "This is an answer");

    let answer = mock.ask(&"fail".into()).await.unwrap_err();
    assert_eq!(format!("{answer}"), "failed");
  }
}
