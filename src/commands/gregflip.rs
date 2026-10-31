use serenity::all::{CreateCommand, ResolvedOption};

pub fn run(_options: &[ResolvedOption]) -> String {
    "GREGS OR TAILS".to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("gregflip").description("Coinflip")
}
