use serenity::all::{CreateCommand, ResolvedOption};

pub fn run(_options: &[ResolvedOption]) -> String {
    "Joooooo, wasap im very kool, test is working!".to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("test").description("very kool test kommand!")
}