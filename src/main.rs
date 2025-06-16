mod commands;

use std::env;
use serenity::all::{Command, CreateAttachment, CreateEmbed, CreateEmbedFooter, CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage, GuildId, Interaction, Timestamp};
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    /*
    async fn message(&self, context: Context, msg: Message) {
        if msg.content == "!messageme" {
            // If the `utils`-feature is enabled, then model structs will have a lot of useful
            // methods implemented, to avoid using an often otherwise bulky Context, or even much
            // lower-level `rest` method.
            //
            // In this case, you can direct message a User directly by simply calling a method on
            // its instance, with the content of the message.
            let builder = CreateMessage::new().content("Bro ich wurde in rust geschrieben und bin der giga gooner");
            let channel = msg.channel_id;
            channel.send_message(&context, builder).await.unwrap();
        }
    }
    */

    /*
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!hello" {
            // The create message builder allows you to easily create embeds and messages using a
            // builder syntax.
            // This example will create a message that says "Hello, World!", with an embed that has
            // a title, description, an image, three fields, and a footer.
            let footer = CreateEmbedFooter::new("This is a footer");
            let embed = CreateEmbed::new()
                .title("This is a title")
                .description("This is a description")
                .image("attachment://ferris_eyes.png")
                .fields(vec![
                    ("This is the first field", "This is a field body", true),
                    ("This is the second field", "Both fields are inline", true),
                ])
                .field("This is the third field", "This is not an inline field", false)
                .footer(footer)
                // Add a timestamp for the current time
                // This also accepts a rfc3339 Timestamp
                .timestamp(Timestamp::now());
            let builder = CreateMessage::new()
                .content("Hello, World!")
                .embed(embed)
                .add_file(CreateAttachment::path("./Untitled.png").await.unwrap());
            let msg = msg.channel_id.send_message(&ctx.http, builder).await;

            if let Err(why) = msg {
                println!("Error sending message: {why:?}");
            }
        }
    }
     */
    
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        println!("Received interaction: {:?}", interaction);
        if let Interaction::Command(command) = interaction {
            // Handle the command interaction
            match command.data.name.as_str() {
                "test" => {
                    
                },
                _ => {
                    let response = CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content("Hiii"));
                    command.create_response(&ctx, response).await.unwrap();
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        for guild in ready.guilds {
            guild.id.set_commands(&ctx.http, vec![
                commands::test::register()
            ]).await.expect("TODO: panic message");

            let guild_command =
                Command::create_global_command(&ctx.http, commands::test::register())
                    .await;
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