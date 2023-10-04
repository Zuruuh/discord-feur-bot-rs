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
use std::{collections::HashMap, env};

const REPLIES_JSON: &str = std::include_str!("../replies.json");
lazy_static! {
    #[derive(Debug)]
    static ref REPLIES: HashMap<String, Vec<String>> = serde_json::from_str(REPLIES_JSON).unwrap();
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
        let content = Regex::new("\\W")
            .unwrap()
            .replace_all(&message.content, "")
            .to_lowercase();
        println!("Parsed message to raw string \"{}\"", &content);

        for keyword in REPLIES.keys() {
            if Regex::new(&format!("{}$", &keyword))
                .unwrap()
                .is_match(&content)
            {
                let replies = REPLIES.get(keyword).unwrap();
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
