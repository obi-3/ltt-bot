mod calc;
mod lexer;
mod parse;
use std::env;

use serenity::async_trait;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::CommandDataOptionValue;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);
            let content = match command.data.name.as_str() {
                "ltt" => {
                    let options = command
                        .data
                        .options
                        .get(0)
                        .expect("Expected string option")
                        .resolved
                        .as_ref()
                        .expect("Expected string object");

                    if let CommandDataOptionValue::String(input) = options {
                        match calc::make_truth_table(input.to_string()) {
                            Ok(s) => s,
                            Err(why) => format!("{why}"),
                        }
                    } else {
                        "Please input logic formula".to_string()
                    }
                }

                _ => "not implemented".to_string(),
            };
            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        dotenv::dotenv().ok();
        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be integer"),
        );

        let _commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands.create_application_command(|command| {
                command
                    .name("ltt")
                    .description("Print truth table by logic formula.")
                    .create_option(|option| {
                        option
                            .name("logic formula")
                            .description("Variables allowed uppercase only.")
                            .kind(CommandOptionType::String)
                            .required(true)
                    })
            })
        })
        .await;

        // println!(
        //     "I now have the following guild slash commands: {:#?}",
        //     commands
        // );
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
