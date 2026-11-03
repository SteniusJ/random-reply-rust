use dotenv;
use serenity::all::ReactionType;
use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::{application::Interaction, channel::Message, gateway::Ready, id::GuildId};
use serenity::prelude::*;
use std::env;

mod commands;
mod db;
mod util;

use util::dice_roll;

// Variable default constants
//
// Default values for reply/react, 1 in N chance
const REPLY_CHANCE: u8 = 6;
const REACT_CHANCE: u8 = 3;

struct Handler<'a> {
    reply_chance: u8,
    react_chance: u8,
    db_host: db::Db_Host<'a>,
}

#[async_trait]
impl<'a> EventHandler for Handler<'a> {
    async fn message(&self, ctx: Context, msg: Message) {
        // Ignore all bot sent messages
        if msg.author.bot {
            return;
        }

        // If mentioned always reply
        if msg.mentions_me(&ctx.http).await.unwrap_or(false) {
            if let Ok(reply) = self.db_host.get_rand_reply().await {
                if let Err(e) = msg.reply(&ctx.http, reply.as_str()).await {
                    println!("Error sending reply: {e:?}");
                }
                return;
            }
            println!("DB Error");
        }

        // Reply if diceroll succeeds
        if dice_roll(self.reply_chance) {
            if let Ok(reply) = self.db_host.get_rand_reply().await {
                if let Err(e) = msg.reply(&ctx.http, reply.as_str()).await {
                    println!("Error sending message: {e:?}");
                }
            }
            println!("DB Error");
        }
        // React if diceroll succeeds
        if dice_roll(self.react_chance) {
            if let Ok(reply) = self.db_host.get_rand_reaction().await {
                if let Err(e) = msg.react(&ctx.http, ReactionType::Unicode(reply)).await {
                    println!("Error reacting: {e:?}");
                }
                return;
            }
            println!("DB Error");
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let content = match command.data.name.as_str() {
                "game" => commands::game::run(&command.data.options(), &self.db_host).await,
                "gregflip" => Some(commands::gregflip::run(&command.data.options())),
                _ => None,
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(e) = command.create_response(&ctx.http, builder).await {
                    println!("Failed to respond to slash command: {e:?}");
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected", ready.user.name);

        let guild_id = GuildId::new(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let commands = guild_id
            .set_commands(
                &ctx.http,
                vec![commands::game::register(), commands::gregflip::register()],
            )
            .await;
        println!("Created the following slash commands: {commands:#?}");
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to read .env file");

    let token = env::var("DISCORD_TOKEN").expect("Expected bot token");
    let reply_chance =
        env::var("REPLY_CHANCE").map_or(REPLY_CHANCE, |v| v.parse().unwrap_or(REPLY_CHANCE));
    let react_chance =
        env::var("REACT_CHANCE").map_or(REACT_CHANCE, |v| v.parse().unwrap_or(REACT_CHANCE));
    let db_url = env::var("DB_URL").expect("Expected database url 'DB_URL' in env");
    let db_password = env::var("DB_PASSWORD").expect("Expected db password 'DB_PASSWORD' in env");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler {
            reply_chance,
            react_chance,
            db_host: db::Db_Host::new(db_url.leak(), db_password.leak()),
        })
        .await
        .expect("Err creating client");

    if let Err(e) = client.start().await {
        println!("Client error: {e:?}");
    }
}
