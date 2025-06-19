use serenity::all::{CommandInteraction, Context, CreateCommand, CreateEmbed, CreateEmbedFooter, CreateInteractionResponse, CreateInteractionResponseMessage, Timestamp};

pub async fn run(command: CommandInteraction, ctx: &Context) {
    let footer = CreateEmbedFooter::new("This is a test footer");
    let embed = CreateEmbed::new()
        .title("This is a test Title")
        .description("This is an important test description, and will be used for testing purposes because of testing its an test and test its an testing")
        .image("https://avatars.githubusercontent.com/u/5430905?s=200&v=4")
        .fields(vec![
            //Not working
            (format!("Ping: {} ms", ctx.http.).as_str(), "my Ping is usually high because i'm a discord bot!", true),
            ("This is the second field", "Both fields are inline", true),
        ])
        .field("This is the third field", "This is not an inline field", false)
        .footer(footer)
        .timestamp(Timestamp::now());
    let response = CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().embed(embed));
    if let Err(e) = command.create_response(ctx, response).await {
        println!("Failed to create response: {:?}", e);
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("info").description("See information about the bot")
}