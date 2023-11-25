use std::env;

use dotenv::dotenv;
use lazy_static::lazy_static;
use regex::Regex;
use serenity::{prelude::GatewayIntents, Client};

mod config;
mod handler;
mod presence;

use config::Config;

pub const AUTHOR_ID: &'static str = "319181790269014016";

lazy_static! {
    pub static ref WHITESPACE_REGEX: Regex = Regex::new("\\W").unwrap();
    pub static ref CONFIG: Config = {
        let raw_replies = std::include_str!("../replies.toml");
        let config = toml::from_str(raw_replies).unwrap();

        config
    };
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let bot_token = env::var("BOT_TOKEN").expect("You need to provide a bot token!");
    env::var("TRASH_ROLE_ID").expect("You need to provide a role identifier!");
    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_PRESENCES;

    let mut client = Client::builder(bot_token, intents)
        .event_handler(handler::Handler)
        .await
        .expect("Error creating client");

    println!("Hello, world!");
    if let Err(reason) = client.start().await {
        println!("An error occured while running the client {:?}", reason);
    }
}
