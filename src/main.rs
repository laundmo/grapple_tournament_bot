use color_eyre::eyre::Result;
use commands::{players, plot};
use dotenvy::dotenv;
use poise::serenity_prelude as serenity;
use request_recurring::create_db_writer;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{env, sync::Arc};

use crate::request_recurring::create_leaderboard_updater;

mod api;
mod commands;
mod recurring;
mod request_recurring;

type Context<'a> = poise::Context<'a, Data, color_eyre::eyre::Error>;

// User data, which is stored and accessible in all command invocations
pub struct Data {
    pool: Option<PgPool>,
}

async fn on_ready(
    ctx: &serenity::Context,
    ready: &serenity::Ready,
    framework: &poise::Framework<Data, color_eyre::eyre::Error>,
) -> Result<Data, color_eyre::eyre::Error> {
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
    let pool: Option<PgPoolOptions> = None;
    #[cfg(any(not(debug_assertions), feature = "prepare"))]
    let pool = Some(
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&std::env::var("DATABASE_URL").expect("missing DATABASE_URL"))
            .await?,
    );
    #[cfg(any(not(debug_assertions), feature = "prepare"))]
    create_db_writer(pool.clone().expect("Pool should only be None when testing")).await?;

    let arc_ctx = Arc::new(ctx.clone());
    create_leaderboard_updater(arc_ctx, ready.user.id).await;

    Ok(Data { pool })
}

async fn error(error: poise::FrameworkError<'_, Data, color_eyre::eyre::Error>) {
    match error {
        poise::FrameworkError::Command { error, ctx } => {
            println!("{:?}", error);
        }
        poise::FrameworkError::CommandCheckFailed { error, ctx } => {
            println!("{:?}", error);
        }
        _ => {
            let _ = poise::builtins::on_error(error).await;
        }
    };
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    color_eyre::install()?;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                players::online(),
                #[cfg(any(not(debug_assertions), feature = "prepare"))]
                plot::plot(),
            ],
            on_error: |err| Box::pin(error(err)),
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, ready, framework| Box::pin(on_ready(ctx, ready, framework)));
    framework.run().await.unwrap();
    Ok(())
}
