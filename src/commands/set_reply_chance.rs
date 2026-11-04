use serenity::all::{
    CommandOptionType, CreateCommand, CreateCommandOption, ResolvedOption, ResolvedValue,
};

pub fn run(options: &[ResolvedOption], reply_chance: &mut u8) -> String {
    if let Some(ResolvedOption {
        value: ResolvedValue::Integer(new_chance),
        ..
    }) = options.first()
    {
        *reply_chance = *new_chance as u8;

        return format!("New reply chance is {}", new_chance);
    }

    "Updating reply chance failed".to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("set_reply_chance")
        .description(
            "Set reply chance to given value. Works like a diceroll 'dice sides' = N sided dice",
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "dice sides",
                "Any positive integer from 1 to 255",
            )
            .min_int_value(1)
            .max_int_value(255)
            .required(true),
        )
}
