use crate::util::dice_roll;
use serenity::all::{CreateCommand, ResolvedOption};

pub fn run(_options: &[ResolvedOption]) -> String {
    if dice_roll(&2) {
        "GREGS".to_string()
    } else {
        "TAILS".to_string()
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("gregflip").description("Coinflip")
}
