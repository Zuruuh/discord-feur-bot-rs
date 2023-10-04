use std::{collections::HashMap, env};

use dotenv::dotenv;
use lazy_static::lazy_static;
use rand::seq::SliceRandom;
use regex::Regex;
use serenity::{
    async_trait,
    model::prelude::Message,
    prelude::{Context, EventHandler, GatewayIntents},
    Client,
};

lazy_static! {
    #[derive(Debug)]
    static ref WHITESPACE_REGEX: Regex = Regex::new("\\W").unwrap();
    static ref REPLIES: HashMap<String, (Regex, Vec<String>)> = {
        let replies_json = std::include_str!("../replies.json");
        let raw_replies: HashMap<String, Vec<String>> = serde_json::from_str(replies_json).unwrap();
        let mut replies_map = HashMap::<String, (Regex, Vec<String>)>::new();

        for (word, replies) in raw_replies.iter() {
            replies_map.insert(
                word.to_owned(),
                (Regex::new(&format!("{word}$")).unwrap(), replies.to_owned())
            );
        }

        drop(raw_replies);

        replies_map
    };
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, message: Message) {
        if message.author.bot {
            return;
        }

        println!(
            "Received message from {} with content \"{}\"",
            &message.author.name, &message.content
        );
        let content = WHITESPACE_REGEX
            .replace_all(&message.content, "")
            .to_lowercase();
        println!("Parsed message to raw string \"{}\"", &content);

        for (_, (regex, replies)) in REPLIES.iter() {
            if regex.is_match(&content) {
                let reply = replies.choose(&mut rand::thread_rng()).unwrap();

                message
                    .reply_ping(&context.http, reply)
                    .await
                    .expect("Could not reply to message!");
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let bot_token = env::var("BOT_TOKEN").expect("You need to provide a bot token!");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(bot_token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    println!("Hello, world!");
    if let Err(reason) = client.start().await {
        println!("An error occured while running the client {:?}", reason);
    }
}
