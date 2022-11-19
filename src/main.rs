use commands::players;
use dotenv::dotenv;
use poise::serenity_prelude as serenity;
use std::env;

mod commands;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

// User data, which is stored and accessible in all command invocations
pub struct Data {}

async fn on_ready(
    ctx: &serenity::Context,
    ready: &serenity::Ready,
    framework: &poise::Framework<Data, Error>,
) -> Result<Data, Error> {
    println!("{} is up!", ready.user.name);

    let appcommands = poise::builtins::create_application_commands(&framework.options().commands);
    let commands = serenity::GuildId::set_application_commands(
        &serenity::GuildId(
            env::var("GUILD_ID")
                .expect("Missing GUILD_ID")
                .parse()
                .expect("GUILD_ID should be a integer"),
        ),
        &ctx.http,
        |commands| {
            *commands = appcommands.clone();
            commands
        },
    )
    .await;

    println!("Added the following slash commands: \n{:#?}", commands);

    Ok(Data {})
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![players::online()],
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(serenity::GatewayIntents::non_privileged())
        .user_data_setup(|ctx, ready, framework| Box::pin(on_ready(ctx, ready, framework)));

    framework.run().await.unwrap();
}
