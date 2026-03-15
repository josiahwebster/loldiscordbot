use reqwest::get;
use serde::{Deserialize, Serialize};
use serenity::all::Message;

struct AiClient {
    http: reqwest::Client,
    api_key: String,
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<String>,
}

#[derive(Deserialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ApiResponse {
    data: String,
}

impl AiClient {
    pub fn new(http: reqwest::Client, api_key: String) -> Self {
        Self { http, api_key }
    }
    pub async fn summarise_matches(&self, data: &String) -> Result<ChatMessage, reqwest::Error> {
        let api_url = "https://api.openai.com/v1";

        let response =
            self.http
                .post(api_url)
                .header("x-api-key", &self.api_key)
                .json(&body)
                .send().await?
                .json::<ChatMessage>()
                .await?;

        return Ok(response);
    }
}
    #[cfg(test)]
    mod tests {
        use super::*;

        #[tokio::test]
        async fn test_ai_response()  {
            let client = reqwest::Client::new();
            let api_key = std::env::var("openapi_key").unwrap();
            let ai = AiClient::new(client, api_key.to_string());

            let result = ai.summarise_matches(&"".to_string()).await;

            assert!(result.is_ok());
        }
    }
