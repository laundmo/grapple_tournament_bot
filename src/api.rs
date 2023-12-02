use std::str::FromStr;

use color_eyre::eyre::Result;

use chrono::{DateTime, FixedOffset, Utc};
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;
use tabled::Tabled;

#[derive(strum_macros::AsRefStr, serde::Deserialize, Debug, EnumString)]
pub(crate) enum Region {
    None,
    AF,
    AN,
    AS,
    EU,
    NA,
    OC,
    SA,
}

impl From<String> for Region {
    fn from(value: String) -> Self {
        Region::from_str(&value).unwrap()
    }
}

fn now_offset_utc() -> DateTime<FixedOffset> {
    Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap())
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct RegionUsers {
    #[serde(default = "now_offset_utc")]
    pub(crate) time: DateTime<FixedOffset>,

    pub(crate) region: Region,

    #[serde(alias = "onlinePlayerCount")]
    pub(crate) amount: i64,
}

pub(crate) async fn get_users() -> Result<Vec<RegionUsers>> {
    Ok(
        reqwest::get("https://gvrfunctions.azurewebsites.net/api/listonlineplayers")
            .await?
            .json()
            .await?,
    )
}

#[derive(Serialize, Deserialize, Debug, Clone, Tabled)]
pub(crate) struct ScoreboardEntry {
    #[tabled(rename = "Name")]
    #[serde(rename = "displayName")]
    pub(crate) display_name: String,
    #[tabled(skip)]
    pub(crate) position: i64,
    #[tabled(rename = "Score")]
    #[serde(rename = "statValue")]
    pub(crate) stat_value: i64,
}

pub(crate) async fn get_leaderboard() -> Result<Vec<ScoreboardEntry>> {
    Ok(
        reqwest::get("https://gvrfunctions.azurewebsites.net/api/getweeklyleaderboard")
            .await?
            .json()
            .await?,
    )
}
