extern crate dotenv;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;
use dotenv::dotenv;
use serde::Deserialize;
use std::env;

mod canvas_api;
use canvas_api::{CanvasApi, PageIterator};

fn env(key: &str) -> String {
    match env::var(key) {
        Ok(val) => val,
        Err(_) => {
            error!("Environmental variable {} not defined", key);
            panic!("Environmental variable {} not defined", key);
        }
    }
}

#[derive(Deserialize, Debug, PartialEq)]
struct Course {
    sis_course_id: Option<String>,
}

struct CourseIterator {
    page_iterator: PageIterator,
    i: std::vec::IntoIter<Course>,
}

impl Iterator for CourseIterator {
    type Item = Course;

    fn next(&mut self) -> Option<Self::Item> {
        // Try to get the next element of "i"
        let element = self.i.next();

        match element {
            Some(course) => return Some(course),
            None => {
                match self.page_iterator.next() {
                    // No more pages left, end iteration
                    None => return None,
                    Some(page) => {
                        self.i = page.unwrap().json::<Vec<Course>>().unwrap().into_iter();

                        return self.i.next();
                    }
                }
            }
        };
    }
}

fn get_courses(account_id: &str) -> CourseIterator {
    let canvas_url = env("CANVAS_API_URL");
    let canvas_token = env("CANVAS_API_TOKEN");

    let api = CanvasApi::new(canvas_url.clone(), canvas_token.clone());
    let page_iterator = api.get_paginated(&format!("/accounts/{}/courses", account_id));

    CourseIterator {
        page_iterator,
        i: Vec::new().into_iter(),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    dotenv().ok();

    for course in get_courses("104") {
        println!("{:?}", course);
    }

    Ok(())
}
