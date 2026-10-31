use serenity::all::ResolvedOption;
use serenity::builder::CreateCommand;

pub fn run(_options: &[ResolvedOption]) -> String {
    "Game message here".to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("game").description("Sends hop on message")
}
