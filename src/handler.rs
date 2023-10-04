use rand::seq::SliceRandom;
use serenity::{
    async_trait,
    model::prelude::{Message, Presence},
    prelude::{Context, EventHandler},
};

use crate::presence;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn presence_update(&self, context: Context, presence: Presence) {
        presence::presence_update(context, presence).await;
    }

    async fn message(&self, context: Context, message: Message) {
        if message.author.bot {
            return;
        }

        println!(
            "Received message from {} with content \"{}\"",
            &message.author.name, &message.content
        );
        let content = crate::WHITESPACE_REGEX
            .replace_all(&message.content, "")
            .to_lowercase();
        println!("Parsed message to raw string \"{}\"", &content);

        for (word, replies) in crate::CONFIG.replies.iter() {
            if content.ends_with(word) {
                let reply = replies.replies.choose(&mut rand::thread_rng()).unwrap();

                message
                    .reply_ping(&context.http, reply)
                    .await
                    .expect("Could not reply to message!");
            }
        }
    }
}
