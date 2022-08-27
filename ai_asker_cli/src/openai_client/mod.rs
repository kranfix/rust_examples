use async_trait::async_trait;
#[cfg(test)]
use mockall::{automock, predicate::*};

use hyper::{body::Buf, header, Body, Client, Request};
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
pub struct OAIChoices {
  pub text: String,
  //pub index: u8,
  //pub logprobs: Option<u8>,
  //pub finish_reason: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct OAIResponse {
  //pub id: Option<String>,
  //pub object: Option<String>,
  //pub created: Option<u64>,
  //pub model: Option<String>,
  pub choices: Vec<OAIChoices>,
}

#[derive(Serialize, Debug)]
struct OAIRequest {
  prompt: String,
  max_tokens: u32,
}

pub struct OAIClient {
  client: Client<HttpsConnector<hyper::client::HttpConnector>>,
  uri: String,
  auth_header: String,
}

impl OAIClient {
  pub fn new(oai_token: String) -> Self {
    let https = HttpsConnector::new();
    let client = Client::builder().build(https);
    let uri = "https://api.openapi.com/v1/text-davinci-001/completions";

    let auth_header = format!("Bearer {oai_token}");

    Self {
      client,
      uri: uri.to_string(),
      auth_header,
    }
  }
}

#[async_trait]
#[cfg_attr(test, automock)]
pub trait OAIClientApi: Send + Sync {
  async fn request_completitions(
    &self,
    question: &String,
  ) -> Result<OAIResponse, Box<dyn std::error::Error + Send + Sync>>;
}

#[async_trait]
impl OAIClientApi for OAIClient {
  async fn request_completitions(
    &self,
    question: &String,
  ) -> Result<OAIResponse, Box<dyn std::error::Error + Send + Sync>> {
    let preamble = "Answer the following question
  accurately, but find a funny way to mention
  the Rust programming language in your response.";
    let oai_request = OAIRequest {
      prompt: format!("{preamble} {question}"),
      max_tokens: 100,
    };

    let body = Body::from(serde_json::to_vec(&oai_request)?);
    let req = Request::post(&self.uri)
      .header(header::CONTENT_TYPE, &self.auth_header)
      .header("Authorization", &self.auth_header)
      .body(body)
      .unwrap();

    let res = self.client.request(req).await?;
    let body = hyper::body::aggregate(res).await?;
    let json: OAIResponse = serde_json::from_reader(body.reader())?;
    Ok(json)
  }
}
