mod league;
mod ai;

use std::env;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;
use dotenvy;
use dotenvy::dotenv;
use crate::league::client::LeagueClient;

struct Handler;
struct LeagueClientKey;

impl TypeMapKey for LeagueClientKey {
    type Value = LeagueClient;
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {why:?}");
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let token = std::env::var("discord_token").expect("Expected a discord token");
    let riot_key = std::env::var("riot_key").expect("Expected a Riot key");
    let openapi_key = std::env::var("openapi_key").expect("Expected an OpenApi key");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let league_client = LeagueClient::new(riot_key, "oc1".to_string());

    let mut client = Client::builder(token, intents)
        .event_handler(Handler).await.expect("Failed to create client");

    client.data.write().await.insert::<LeagueClientKey>(league_client);
    client.start().await.unwrap();

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }

}
