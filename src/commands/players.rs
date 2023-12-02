use crate::Context;
use color_eyre::eyre::Result;

/// List online players
#[poise::command(prefix_command, slash_command, aliases("players"))]
pub(crate) async fn online(ctx: Context<'_>) -> Result<()> {
    ctx.send(|m| {
        m.content(format!(
            "This command is gone, please look at {}",
            std::env::var("ONLINE_CHANNEL")
                .map(|s| format!("<#{}>", s))
                .unwrap_or("the online-players channel".to_string())
        ))
        .ephemeral(true)
    })
    .await?;
    Ok(())
}
