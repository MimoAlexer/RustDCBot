use serenity::all::{CommandInteraction, Context, CreateCommand, CreateEmbed, CreateEmbedFooter, CreateInteractionResponse, CreateInteractionResponseMessage, InteractionContext, ResolvedOption, Timestamp};

pub async fn run(command: CommandInteraction, ctx: &Context) {
    let footer = CreateEmbedFooter::new("This is a test footer");
    let embed = CreateEmbed::new()
        .title("This is a test Title")
        .description("This is an important test description, and will be used for testing purposes because of testing its an test and test its an testing")
        .image("https://avatars.githubusercontent.com/u/5430905?s=200&v=4")
        .fields(vec![
            ("This is the first field", "This is a field body", true),
            ("This is the second field", "Both fields are inline", true),
        ])
        .field("This is the third field", "This is not an inline field", false)
        .footer(footer)
        // Add a timestamp for the current time
        // This also accepts a rfc3339 Timestamp
        .timestamp(Timestamp::now());
    let response = CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().embed(embed));
    command.create_response(ctx, response).await.unwrap();
}

pub fn register() -> CreateCommand {
    CreateCommand::new("info").description("See information about the bot")
}