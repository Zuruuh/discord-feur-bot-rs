use std::env;

use serenity::{
    model::prelude::{Presence, RoleId},
    prelude::Context,
};

pub async fn presence_update(context: Context, presence: Presence) {
    if presence.guild_id.is_none() {
        return;
    }

    let guild = presence.guild_id.unwrap();
    if let Ok(allowed_guild_id) = env::var("GUILD_ID") {
        if allowed_guild_id.parse::<u64>().unwrap().ne(&guild.0) {
            return;
        }
    }
    let member = guild.member(&context.http, presence.user.id).await;

    if member.is_err() {
        println!(
            "Could not find member with id {} for guild {}",
            presence.user.id, guild.0
        );

        return;
    }
    let mut member = member.unwrap();

    let trash_role_id: RoleId = RoleId::from(
        env::var("TRASH_ROLE_ID")
            .unwrap()
            .parse::<u64>()
            .expect("Invalid role ID provided"),
    );

    let roles = guild
        .roles(&context.http)
        .await
        .expect("Could not read roles");

    roles
        .get(&trash_role_id)
        .expect(&format!("Role with id {trash_role_id} does not exist"));

    let should_be_trashed = presence.activities.iter().any(|activity| {
        crate::CONFIG
            .banned_games
            .iter()
            .any(|banned_game| -> bool {
                activity
                    .name
                    .to_lowercase()
                    .contains(&banned_game.to_lowercase())
            })
    });

    let has_trash_role = member.roles.contains(&trash_role_id);

    if should_be_trashed && !has_trash_role {
        member
            .add_role(&context.http, &trash_role_id)
            .await
            .expect(&format!("Could not add trash role to {}", member.user.id));

        println!("Adding trash role to {}!", member.user.id);
    } else if !should_be_trashed && has_trash_role {
        member
            .remove_role(&context.http, &trash_role_id)
            .await
            .expect(&format!(
                "Could not remove trash role to {}",
                member.user.id
            ));

        println!("Removing trash role to {}!", member.user.id);
    }
}
