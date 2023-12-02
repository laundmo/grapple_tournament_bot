use std::{fmt::Display, str::FromStr};

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

impl Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Region::*;
        f.write_str(match *self {
            AF => "Africa",
            AN => "Antarctica",
            AS => "Asia",
            EU => "Europe",
            NA => "North-America",
            OC => "Oceania",
            SA => "South America",
            None => "None",
        })
    }
}

impl From<String> for Region {
    fn from(value: String) -> Self {
        Region::from_str(&value).unwrap()
    }
}

fn now_offset_utc() -> DateTime<FixedOffset> {
    Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap())
}

#[derive(serde::Deserialize, Debug, Tabled)]
pub(crate) struct RegionUsers {
    #[tabled(skip)]
    #[serde(default = "now_offset_utc")]
    pub(crate) time: DateTime<FixedOffset>,

    #[tabled(rename = "Region")]
    pub(crate) region: Region,

    #[tabled(rename = "Players")]
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
