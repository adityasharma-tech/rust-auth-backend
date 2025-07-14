use std::collections::HashMap;
use serde::Deserialize;

pub mod env;
mod error_handler;
pub mod response;
pub mod types;

#[derive(Deserialize)]
pub struct FetchLocation {
    pub ip: Option<String>,
    pub city: Option<String>,
    pub region: Option<String>,
    pub country: Option<String>,
    pub loc: Option<String>,
    pub org: Option<String>,
    pub postal: Option<String>,
    pub timezone: Option<String>
}

pub async fn fetch_location_data(ip: String) -> Result<FetchLocation, ()> {
    let request = reqwest::get(format!("https://ipinfo.io/{}/json", ip))
        .await.map_err(|_| ())?
        .json::<FetchLocation>()
        .await.map_err(|_| ())?;

    Ok(request)
}