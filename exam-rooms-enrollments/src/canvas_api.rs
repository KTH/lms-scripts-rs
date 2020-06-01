use serde::Deserialize;
use reqwest::blocking::{Client, Response};

pub struct CanvasApi {
    canvas_url: String,
    canvas_token: String,
    client: Client,
}

#[derive(Deserialize, Debug)]
struct Course {
    sis_course_id: Option<String>,
}

pub struct PageIterator<'a> {
    canvas_api: &'a CanvasApi,
    next_url: Option<String>,
}

impl<'a> Iterator for PageIterator<'a> {
    type Item = Response;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

impl CanvasApi {
    pub fn new(canvas_url: String, canvas_token: String) -> CanvasApi {
        CanvasApi {
            canvas_token,
            canvas_url,
            client: Client::new(),
        }
    }

    pub fn get(&self, endpoint: &str) -> Result<Response, reqwest::Error> {
        self.client
            .get(&format!("{}{}", self.canvas_url, endpoint))
            .bearer_auth(&self.canvas_token)
            .send()
    }

    pub fn get_paginated(&self, endpoint: &str) -> PageIterator {
        PageIterator{
            canvas_api: &self,
            next_url: Some(format!("{}{}", self.canvas_url, endpoint))
        }
    }

    pub fn get_courses(&self, account_id: &str) -> Result<Vec<Course>, reqwest::Error> {
        let response = self
            .get(&format!("/accounts/{}/courses", account_id))?;

        println!("{:?}", response.headers().get("link"));

        response.json::<Vec<Course>>()
    }
}
