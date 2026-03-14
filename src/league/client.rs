use reqwest::Client;
use reqwest::Error;

pub struct LeagueClient {
    http: Client,
    api_key: String,
    region: String,
}

pub struct Summoner {
    pub id: String,
    pub game_name: String,
    pub tag_line: u64,
}

impl LeagueClient {
    pub fn new(api_key: String, region: String) -> Self {
        Self {
            http: Client::new(),
            api_key,
            region,
        }
    }

}