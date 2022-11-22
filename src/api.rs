use miette::{Diagnostic, Result};
use thiserror::Error;

#[derive(strum_macros::AsRefStr, serde::Deserialize, Debug)]
pub(crate) enum Region {
    AF, // Afghanistan
    AN, // 
    AS,
    EU,
    NA,
    OC,
    SA,
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct Users {
    pub(crate) region: Region,
    #[serde(alias = "onlinePlayerCount")]
    pub(crate) online_player_count: u32,
}

#[derive(Error, Diagnostic, Debug)]
pub(crate) enum ApiError {
    #[error(transparent)]
    #[diagnostic(code(gt_bot::api))]
    ReqwestErr(#[from] reqwest::Error),
}

pub(crate) async fn get_users() -> Result<Vec<Users>, ApiError> {
    let response =
        reqwest::get("https://gvrfunctions.azurewebsites.net/api/listonlineplayers").await?;
    let users: Vec<Users> = response.json().await?;
    Ok(users)
}
