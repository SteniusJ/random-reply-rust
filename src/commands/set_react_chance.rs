use serenity::all::{
    CommandOptionType, CreateCommand, CreateCommandOption, ResolvedOption, ResolvedValue,
};

pub fn run(options: &[ResolvedOption], react_chance: &mut u8) -> String {
    if let Some(ResolvedOption {
        value: ResolvedValue::Integer(new_chance),
        ..
    }) = options.first()
    {
        *react_chance = *new_chance as u8;

        return format!("New react chance is {}", new_chance);
    }

    "Updating react chance failed".to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("set_react_chance")
        .description(
            "Set react chance to given value. Works like a diceroll 'dice sides' = N sided dice",
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
