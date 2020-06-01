extern crate pretty_env_logger;
extern crate dotenv;
#[macro_use] extern crate log;
use dotenv::dotenv;
use std::env;
use serde::{Deserialize};

fn env(key: &str) -> String {
    match env::var(key) {
        Ok(val) => val,
        Err(_) => {
            error!("Environmental variable {} not defined", key);
            panic!("Environmental variable {} not defined", key);
        }
    }
}

#[derive(Deserialize, Debug)]
struct Course {
    sis_course_id: Option<String>
}

async fn get_courses (account_id: &str) -> Result<Vec<Course>, reqwest::Error> {
    let canvas_url = env("CANVAS_API_URL");
    let canvas_token = env("CANVAS_API_TOKEN");

    let client = reqwest::Client::new();
    let response = client.get(&format!("{}/accounts/{}/courses", canvas_url, account_id))
        .bearer_auth(canvas_token)
        .send()
        .await?;

    println!("{:?}", response.headers().get("link"));

    response
        .json::<Vec<Course>>()
        .await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    dotenv().ok();

    let courses = get_courses("104").await?;

    println!("{:?}", courses);

    Ok(())
}
