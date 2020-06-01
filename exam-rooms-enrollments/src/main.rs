extern crate dotenv;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;
use dotenv::dotenv;
use regex::Regex;
use serde::de::DeserializeOwned;
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
    id: i32,
    sis_course_id: Option<String>,
    workflow_state: String,
    name: String,
}

#[derive(Deserialize, Debug, PartialEq)]
struct User {
    sortable_name: Option<String>,
    login_id: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq)]
struct Enrollment {
    id: i32,
    sis_user_id: Option<String>,
    role: String,
    user: User,
}

struct CanvasIterator<T> {
    page_iterator: PageIterator,
    i: std::vec::IntoIter<T>,
}

impl<T: DeserializeOwned> Iterator for CanvasIterator<T> {
    type Item = T;

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
                        self.i = page.unwrap().json::<Vec<Self::Item>>().unwrap().into_iter();

                        return self.i.next();
                    }
                }
            }
        };
    }
}

fn get_courses(account_id: &str) -> CanvasIterator<Course> {
    let canvas_url = env("CANVAS_API_URL");
    let canvas_token = env("CANVAS_API_TOKEN");

    let api = CanvasApi::new(canvas_url.clone(), canvas_token.clone());
    let page_iterator = api.get_paginated(&format!("/accounts/{}/courses", account_id));

    CanvasIterator::<Course> {
        page_iterator,
        i: Vec::new().into_iter(),
    }
}

fn get_enrollments (course_id: i32) -> CanvasIterator<Enrollment> {
    let canvas_url = env("CANVAS_API_URL");
    let canvas_token = env("CANVAS_API_TOKEN");

    let api = CanvasApi::new(canvas_url.clone(), canvas_token.clone());
    let page_iterator = api.get_paginated(&format!("/courses/{}/enrollments", course_id));

    CanvasIterator::<Enrollment> {
        page_iterator,
        i: Vec::new().into_iter(),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    dotenv().ok();

    let re = Regex::new(r"^AKT\.([\w-]+)\.(\d\d\d\d-\d\d-\d\d)$").unwrap();

    let courses = get_courses("104")
        .filter(|course| course.sis_course_id.is_some())
        .filter(|course| course.workflow_state != "unpublished")
        .filter(|course| re.is_match(&course.sis_course_id.as_ref().unwrap()));

    for course in courses {
        let enrollments = get_enrollments(course.id)
            .filter(|e| e.sis_user_id.is_some());

        for enrollment in enrollments {
            println!("{:?}", enrollment);
        }
    }

    Ok(())
}
