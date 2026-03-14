use reqwest::Client;
use reqwest::Error;
use crate::league::models::Summoner;

pub struct LeagueClient {
    http: Client,
    api_key: String,
    region: String,
}

impl LeagueClient {
    pub fn new(api_key: String, region: String) -> Self {
        Self {
            http: Client::new(),
            api_key,
            region,
        }
    }

    pub async fn get_summoner() {

    }

    pub async fn get_recent_matches(&self, match_count: i32) -> Result<Summoner, Error> {

        let url = format!("https:///{}", self.region);

        let summoner = self.http
            .get(url)
            .header("X-API-KEY", &self.api_key)
            .send()
            .await?
            .json::<Summoner>()
            .await?;

        Ok(summoner)

    }

}