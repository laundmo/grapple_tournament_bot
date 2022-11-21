use crate::{Context, Error};
use serde;

#[derive(strum_macros::AsRefStr, serde::Deserialize, Debug)]
enum Region {
    AF,
    AN,
    AS,
    EU,
    NA,
    OC,
    SA,
}

#[derive(serde::Deserialize, Debug)]
struct Users {
    region: Region,
    #[serde(alias = "onlinePlayerCount")]
    online_player_count: u32,
}

#[poise::command(prefix_command, slash_command, aliases("players"))]
pub async fn online(ctx: Context<'_>) -> Result<(), Error> {
    let response =
        reqwest::get("https://gvrfunctions.azurewebsites.net/api/listonlineplayers").await?;
    let users: Vec<Users> = response.json().await?;
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
