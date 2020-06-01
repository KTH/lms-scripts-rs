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

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    dotenv().ok();

    let canvas_url = env("CANVAS_API_URL");
    // let canvas_token = env("CANVAS_API_TOKEN");

    let response = reqwest::get(&format!("{}{}", canvas_url, "/accounts"))
        .await;

    println!("{:?}", response);
}
