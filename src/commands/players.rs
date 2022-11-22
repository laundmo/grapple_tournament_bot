use crate::{api, BotError, Context};
use miette::Result;

#[poise::command(prefix_command, slash_command, aliases("players"))]
pub(crate) async fn online(ctx: Context<'_>) -> Result<(), BotError> {
    let users = api::get_users().await?;
    if users.is_empty() {
        ctx.send(|m| {
            m.embed(|b| {
                b.title("Grapple Tournament stats")
                    .description("No players online")
            })
        })
        .await?;
    } else {
        ctx.send(|m| {
            m.embed(|b| {
                b.title("Grapple Tournament stats")
                    .description(format!(
                        "Total Players: {}",
                        users.iter().map(|u| u.online_player_count).sum::<u32>()
                    ))
                    .fields(vec![
                        (
                            "Region",
                            users
                                .iter()
                                .map(|u| u.region.as_ref().to_string())
                                .collect::<Vec<String>>()
                                .join("\n"),
                            true,
                        ),
                        (
                            "Online",
                            users
                                .iter()
                                .map(|u| format!("{}", u.online_player_count))
                                .collect::<Vec<String>>()
                                .join("\n"),
                            true,
                        ),
                    ])
                    .footer(|f| f.text("Including players in private matches"))
            })
        })
        .await?;
    }
    Ok(())
}
