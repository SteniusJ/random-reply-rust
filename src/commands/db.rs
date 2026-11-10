use std::convert::identity;

use crate::db::*;
use serenity::builder::*;
use serenity::model::prelude::*;
use serenity::prelude::*;

const ACCEPTED_ANSWERS: [&str; 3] = ["reply", "react", "game"];

enum Interaction<'a> {
    CommandInteraction(&'a CommandInteraction),
    ComponentInteraction(&'a ComponentInteraction),
}

pub async fn run<'a>(
    options: &[ResolvedOption<'a>],
    host: &DbHost<'a>,
    ctx: &Context,
    interaction: &CommandInteraction,
) -> Result<(), DbError> {
    /* NOTE!
    May want to change it so that this function no longer returns DbError
    */
    if let Some(ResolvedOption {
        value: ResolvedValue::String(db_name),
        ..
    }) = options.first()
    {
        let builder = build_db_list_message(
            *db_name,
            0,
            host,
            Interaction::CommandInteraction(interaction),
        )
        .await?;

        if let Err(err) = interaction.create_response(&ctx.http, builder).await {
            println!("Could not respond: {err}");
        }

        // Command doesn't need to return a string, rethink this
        return Ok(());
    }

    Err(DbError::Err("".to_string()))
}

pub async fn component<'a>(
    interaction: &ComponentInteraction,
    id_chunks: &[&str],
    host: &DbHost<'a>,
) -> Result<(), DbError> {
    // We store the user_id and interaction_id in the id
    // These should be used to check that the operation is performed
    // by a authorized user for this session.
    let db_name = id_chunks[3];
    let current_page = 0; // <- this should also be fetched from some in memory cache

    // Match specific command from id
    match id_chunks[4] {
        "previous" => (), // same here as bellow just with -1
        "next" => {
            let builder = build_db_list_message(
                db_name,
                current_page + 1,
                host,
                Interaction::ComponentInteraction(interaction),
            )
            .await?;

            // then update modal message
        }
        _ => (),
    }

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("db")
        .description("Read and modify database entries")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "database name",
                "name of database you want to interact with",
            )
            .add_string_choice("reply database", "reply")
            .add_string_choice("reaction database", "react")
            .add_string_choice("game database", "game")
            .required(true),
        )
}

fn is_accepted_answer(ans: &str) -> bool {
    for acc_ans in ACCEPTED_ANSWERS {
        if ans == acc_ans {
            return true;
        }
    }
    false
}

async fn build_db_list_message<'a>(
    db_name: &str,
    page: u64,
    host: &DbHost<'a>,
    interaction: Interaction<'a>,
) -> Result<CreateInteractionResponse, DbError> {
    if !is_accepted_answer(db_name) {
        // end command here if answer is not accepted
        return Err(DbError::Err("Not accepted entry".to_string()));
    }

    let db_contents = match db_name {
        "reply" => host.get_reply_page(page).await?,
        "react" => host.get_react_page(page).await?,
        "game" => host.get_game_page(page).await?,
        _ => vec![], // NOT reachable at this point
    };

    let message = db_contents
        .iter()
        .map(|(id, msg)| format!("#{id} {msg}"))
        .collect::<Vec<String>>()
        .join("\n");

    let user_id = match interaction {
        Interaction::CommandInteraction(interaction) => interaction.user.id,
        Interaction::ComponentInteraction(interaction) => interaction.user.id,
    };
    let interaction_id = match interaction {
        Interaction::CommandInteraction(interaction) => interaction.id,
        Interaction::ComponentInteraction(interaction) => interaction.id,
    };

    let components = vec![
        CreateActionRow::Buttons(vec![
            CreateButton::new(format!("{user_id}:{interaction_id}:db:{db_name}:previous"))
                .label("Previous")
                .style(ButtonStyle::Primary),
            CreateButton::new(format!("{user_id}:{interaction_id}:db:{db_name}:next"))
                .label("Next")
                .style(ButtonStyle::Primary),
        ]),
        // Maybe use this?
        //CreateActionRow::SelectMenu(CreateSelectMenu::new(custom_id, kind)),
    ];

    let builder = CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
            .content(message)
            .components(components)
            .ephemeral(true),
    );

    Ok(builder)
}
