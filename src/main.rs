use commands::players;
use dotenvy::dotenv;
use miette::Diagnostic;
use poise::serenity_prelude as serenity;
use request_recurring::create_db_writer;
use std::env;
use thiserror::Error;

mod api;
mod commands;
mod recurring;
mod request_recurring;

#[derive(Error, Diagnostic, Debug)]
pub(crate) enum BotError {
    #[error(transparent)]
    #[diagnostic(code(gt_bot::api))]
    SerenityErr(#[from] serenity::SerenityError),
    #[error(transparent)]
    #[diagnostic(code(gt_bot::api))]
    ApiError(#[from] api::ApiError),
    #[error(transparent)]
    #[diagnostic(code(gt_bot::api))]
    SqlxError(#[from] sqlx::Error),
    #[error(transparent)]
    #[diagnostic(code(gt_bot::api))]
    SqlxMigrateError(#[from] sqlx::migrate::MigrateError),
}

type Context<'a> = poise::Context<'a, Data, BotError>;

// User data, which is stored and accessible in all command invocations
pub struct Data {}

async fn on_ready(
    ctx: &serenity::Context,
    ready: &serenity::Ready,
    framework: &poise::Framework<Data, BotError>,
) -> Result<Data, BotError> {
    println!("{} is up!", ready.user.name);

    let commands_b = poise::builtins::create_application_commands(&framework.options().commands);

    match env::var("GUILD_ID").ok().and_then(|e| e.parse().ok()) {
        Some(guild_id) => {
            let commands = serenity::GuildId::set_application_commands(
                &serenity::GuildId(guild_id),
                &ctx.http,
                |commands| {
                    *commands = commands_b.clone();
                    commands
                },
            )
            .await;
            println!("Added the following slash commands: \n{:#?}", commands);
        }
        None => {
            let global_command =
                serenity::Command::set_global_application_commands(&ctx.http, |commands| {
                    *commands = commands_b;
                    commands
                })
                .await;
            println!(
                "Added the following guild slash commands: \n{:#?}",
                global_command
            );
        }
    }

    Ok(Data {})
}

async fn error<'a>(error: poise::FrameworkError<'a, Data, BotError>) {
    match error {
        poise::FrameworkError::Command { error, ctx } => {
            println!("{:?}", error);
        }
        poise::FrameworkError::CommandCheckFailed { error, ctx } => {
            println!("{:?}", error);
        }
        _ => {
            poise::builtins::on_error(error).await;
        }
    };
}

#[tokio::main]
async fn main() -> Result<(), BotError> {
    dotenv().ok();
    let begin: i16 = 1234;
    let bytes1 = begin.to_ne_bytes();
    let bytes2 = begin.to_be_bytes();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![players::online()],
            on_error: |err| Box::pin(error(err)),
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(serenity::GatewayIntents::non_privileged())
        .user_data_setup(|ctx, ready, framework| Box::pin(on_ready(ctx, ready, framework)));
    create_db_writer().await?;
    framework.run().await.unwrap();
    Ok(())
}
