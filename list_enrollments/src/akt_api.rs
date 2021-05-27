use chrono::NaiveDate;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::time::Duration;
use url::Url;

#[derive(Deserialize, Debug, Clone)]
pub struct Response {
    pub aktivitetstillfallen: Vec<Aktivitetstillfalle>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Aktivitetstillfalle {
    #[serde(rename = "ladokUID")]
    pub ladok_uid: String,
}

/// Get all the course rounds in a given term that starts in a given period
pub fn get_aktivitetstillfallen(
    akt_url: &str,
    akt_token: &str,
    date: &NaiveDate,
) -> impl Iterator<Item = Aktivitetstillfalle> {
    let url_with_slash = match akt_url.ends_with('/') {
        true => akt_url.to_string(),
        false => format!("{}/", akt_url),
    };

    let suffix = format!(
        "aktivitetstillfallen/students?fromDate={}&toDate={}",
        date, date
    );
    let full_url = Url::parse(&url_with_slash).unwrap().join(&suffix).unwrap();

    let client = Client::builder()
        .timeout(Duration::from_secs(300))
        .build()
        .unwrap();

    client
        .get(full_url)
        .header("canvas_api_token", akt_token)
        .send()
        .unwrap()
        .json::<Response>()
        .unwrap()
        .aktivitetstillfallen
        .into_iter()
}
