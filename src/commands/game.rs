use crate::db::DbHost;
use serenity::all::ResolvedOption;
use serenity::builder::CreateCommand;

pub async fn run<'a>(_options: &[ResolvedOption<'a>], host: &DbHost<'a>) -> Option<String> {
    if let Ok(reply) = host.get_rand_game().await {
        return Some(reply);
    }
    return None;
}

pub fn register() -> CreateCommand {
    CreateCommand::new("game").description("Sends hop on message")
}
