mod commands;

use crate::commands::info;
use serenity::all::{Command, CreateInteractionResponse, CreateInteractionResponseMessage, Interaction};
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::env;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        for guild in ready.guilds {
            let commands = guild.id.get_commands(&ctx.http).await.unwrap();
            for command in commands {
                guild.id.delete_command(&ctx.http, command.id).await.unwrap();
            }
            guild.id.set_commands(&ctx.http, vec![
                commands::info::register()
            ]).await.expect("TODO: panic message");

            let guild_command =
                Command::create_global_command(&ctx.http, commands::info::register())
                    .await;
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            println!("Received interaction from: {}", ctx.http.get_user(command.user.id).await.unwrap().name);
            match command.data.name.as_str() {
                "info" => {
                    info::run(command, &ctx).await;
                },
                _ => {
                    let response = CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(format!("I... dont really understand {}", command.data.name.as_str())));
                    command.create_response(&ctx, response).await.unwrap();
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");
    
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}