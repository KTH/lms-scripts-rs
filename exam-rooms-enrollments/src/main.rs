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

fn get_courses(account_id: &str) -> () {
    let canvas_url = env("CANVAS_API_URL");
    let canvas_token = env("CANVAS_API_TOKEN");

    let api = CanvasApi::new(canvas_url.clone(), canvas_token.clone());

    let pages = api.get_paginated(&format!("/accounts/{}/courses", account_id));

    let courses = pages.flat_map(|response| response.unwrap().json::<Vec<Course>>().unwrap());

    for course in courses {
        println!("{:?}", course);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    dotenv().ok();

    get_courses("104");

    Ok(())
}
