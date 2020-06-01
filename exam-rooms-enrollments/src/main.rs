extern crate dotenv;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;
use dotenv::dotenv;
use serde::Deserialize;
use std::env;

mod canvas_api;
use canvas_api::CanvasApi;

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
    sis_course_id: Option<String>,
}

fn get_courses(account_id: &str) -> Result<Vec<Course>, reqwest::Error> {
    let canvas_url = env("CANVAS_API_URL");
    let canvas_token = env("CANVAS_API_TOKEN");

    let api = CanvasApi::new(canvas_url.clone(), canvas_token.clone());

    // api.get_paginated(&format!("/accounts/{}/courses", account_id)).await?;

    let response = api.get(&format!("/accounts/{}/courses", account_id))?;

    println!("{:?}", response.headers().get("link"));

    response.json::<Vec<Course>>()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    dotenv().ok();

    let courses = get_courses("104")?;

    println!("{:?}", courses);

    Ok(())
}
