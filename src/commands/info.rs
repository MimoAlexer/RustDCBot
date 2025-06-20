use std::sync::Arc;
use std::time::Duration;
use serenity::all::{CommandInteraction, Context, CreateCommand, CreateEmbed, CreateEmbedFooter, CreateInteractionResponse, CreateInteractionResponseMessage, Interaction, ShardManager, Timestamp};
use serenity::prelude::TypeMapKey;

struct ShardManagerContainer;
impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<ShardManager>;
}


pub async fn run(command: CommandInteraction, ctx: &Context) {
    let data = ctx.data.read().await;
    let shard_manager = data.get::<ShardManagerContainer>()
        .expect("ShardManager not found in Context.data")
        .clone();

    let latency_opt = {
        let runners = shard_manager.runners.lock().await;
        runners.get(&ctx.shard_id).and_then(|runner_info| runner_info.latency)
    };

    let ping_title = if let Some(latency) = latency_opt {
        format!("Ping: {:.3} s", latency.as_secs_f64())
    } else {
        "Ping: N/A".to_string()
    };

    // Build the embed with the ping field and other fields
    let embed = CreateEmbed::new()
        .title("This is a test Title")
        .description("This is an important test description, used for testing purposes.")
        .image("https://avatars.githubusercontent.com/u/5430905?s=200&v=4")
        .field(ping_title, "My ping is usually high because I'm a Discord bot!", true)
        .field("This is the second field", "Both fields are inline", true)
        .field("This is the third field", "This is not an inline field", false)
        .footer(CreateEmbedFooter::new("This is a test footer"))
        .timestamp(Timestamp::now());

    let response = CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new().embed(embed)
    );
    if let Err(e) = command.create_response(ctx, response).await {
        println!("Failed to create response: {:?}", e);
    }
}


pub fn register() -> CreateCommand {
    CreateCommand::new("info").description("See information about the bot")
}