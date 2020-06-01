extern crate pretty_env_logger;
extern crate dotenv;
#[macro_use] extern crate log;
use dotenv::dotenv;
use std::env;

fn env(key: &str) -> String {
    match env::var(key) {
        Ok(val) => val,
        Err(_) => {
            error!("Environmental variable {} not defined", key);
            panic!("Environmental variable {} not defined", key);
        }
    }
}

async fn get_courses (account_id: &str) -> Result<reqwest::Response, reqwest::Error> {
    let canvas_url = env("CANVAS_API_URL");
    let canvas_token = env("CANVAS_API_TOKEN");

    let client = reqwest::Client::new();
    client.get(&format!("{}/accounts/{}/courses", canvas_url, account_id))
        .bearer_auth(canvas_token)
        .send()
        .await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    dotenv().ok();

    let response = get_courses("104").await?;

    println!("{:?}", response);

    Ok(())
}
