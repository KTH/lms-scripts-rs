use reqwest::blocking::{Client, Response};

pub struct CanvasApi {
    canvas_url: String,
    canvas_token: String,
    client: Client,
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
        PageIterator {
            canvas_api: &self,
            next_url: Some(format!("{}{}", self.canvas_url, endpoint)),
        }
    }
}
