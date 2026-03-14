use reqwest::get;
use serde::Deserialize;
use serenity::all::Message;

struct AiClient {
    http: reqwest::Client,
    api_key: String,
}

#[derive(Deserialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
}
impl AiClient {

    pub fn new(http: reqwest::Client, api_key: String) -> Self {
        Self { http, api_key }
    }
    pub async fn summarise_matches(&self, data: &String) -> Result<ChatRequest, reqwest::Error> {
        let api_url = "https://api.openai.com/v1";

        let response =
            self.http
                .post(api_url)
            .header("x-api-key", &self.api_key)
            .json(&body)
        .send().await?
                .json::<ChatRequest>()
                .await?;




    }
}
