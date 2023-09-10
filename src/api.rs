use std::str::FromStr;

use color_eyre::eyre::Result;

use chrono::{DateTime, FixedOffset, Utc};
use strum_macros::EnumString;

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
    let response =
        reqwest::get("https://gvrfunctions.azurewebsites.net/api/listonlineplayers").await?;
    let users: Vec<RegionUsers> = response.json().await?;
    Ok(users)
}
