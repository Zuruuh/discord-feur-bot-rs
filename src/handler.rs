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

        if message.content.starts_with("+say") && message.author.id.to_string() == crate::AUTHOR_ID
        {
            message.delete(&context).await.unwrap();

            message
                .channel(&context)
                .await
                .unwrap()
                .id()
                .send_message(&context, |builder| {
                    builder.content(message.content.chars().skip(4).collect::<String>())
                })
                .await
                .unwrap();

            return;
        }

        let content = crate::WHITESPACE_REGEX
            .replace_all(&message.content, "")
            .to_lowercase();

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
