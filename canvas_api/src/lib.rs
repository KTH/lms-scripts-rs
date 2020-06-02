//! Helper functions to interact with the Canvas API
//!
//! This package contains functions to perform GET requests to the Canvas LMS
//! API and helper functions to deal with things like pagination.
use reqwest::blocking::{Client, Response};
use serde::de::DeserializeOwned;

/// Instance of a Canvas client. Contains the Canvas URL and the access token.
#[derive(Clone)]
pub struct CanvasApi {
    canvas_url: String,
    canvas_token: String,
    client: Client,
}

/// Iterator for pages. You use it to traverse through pages in a paginated GET
/// request. [Read more about paginated requests in Canvas](https://canvas.instructure.com/doc/api/file.pagination.html)
pub struct PageIterator {
    canvas_api: CanvasApi,
    next_url: Option<String>,
}

/// Iterator for items. In requests that returns multiple items, this iterator
/// helps traversing every item. It requests the following pages automatically.
pub struct ItemIterator<T> {
    page_iterator: PageIterator,
    i: std::vec::IntoIter<T>,
}

fn get_next_from_link(link: &str) -> Option<String> {
    let next_link = link.split(',').find(|&x| x.ends_with("rel=\"next\""));

    let next_url = match next_link {
        None => return None,
        Some(next_link) => next_link
            .trim_start_matches("<")
            .trim_end_matches(">; rel=\"next\""),
    };

    Some(next_url.to_string())
}

fn get_next_url(response: &Result<Response, reqwest::Error>) -> Option<String> {
    let response = match response {
        Err(_) => return None,
        Ok(r) => r,
    };

    let link = match response.headers().get("link") {
        None => return None,
        Some(link) => match link.to_str() {
            Err(_) => return None,
            Ok(l) => l,
        },
    };

    get_next_from_link(link)
}

impl Iterator for PageIterator {
    type Item = Result<Response, reqwest::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let client = &self.canvas_api.client;

        if self.next_url == None {
            return None;
        }

        let response = client
            .get(self.next_url.as_ref().unwrap())
            .bearer_auth(&self.canvas_api.canvas_token)
            .send();

        self.next_url = get_next_url(&response);

        Some(response)
    }
}

impl PageIterator {
    pub fn items<T>(self) -> ItemIterator<T> {
        ItemIterator::<T> {
            page_iterator: self,
            i: Vec::new().into_iter(),
        }
    }
}

impl<T: DeserializeOwned> Iterator for ItemIterator<T> {
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

impl CanvasApi {
    /// Creates a new CanvasApi instance by giving the URL and an access token.
    ///
    /// Example:
    ///
    /// ```
    /// use canvas_api::CanvasApi;
    ///
    /// let api = CanvasApi::new("https://kth.test.instructure.com", "XXXX");
    /// ```
    pub fn new(canvas_url: String, canvas_token: String) -> CanvasApi {
        CanvasApi {
            canvas_token,
            canvas_url,
            client: Client::new(),
        }
    }

    /// Performs a GET request to and endpoint in Canvas. This function does
    /// not handle any error
    ///
    /// Example:
    ///
    /// ```
    /// use canvas_api::CanvasApi;
    ///
    /// let api = CanvasApi::new("https://kth.test.instructure.com", "XXXX");
    /// api.get("/accounts/1")
    /// ```
    pub fn get(&self, endpoint: &str) -> Result<Response, reqwest::Error> {
        self.client
            .get(&format!("{}{}", self.canvas_url, endpoint))
            .bearer_auth(&self.canvas_token)
            .send()
    }

    /// Returns an iterator that can be used to perform requests to a paginated
    /// endpoint.
    ///
    /// See the documentation of [`PageIterator`] to learn how to traverse
    /// through pages
    ///
    /// [`PageIterator`]: struct.PageIterator.html
    pub fn get_paginated(&self, endpoint: &str) -> PageIterator {
        PageIterator {
            canvas_api: self.clone(),
            next_url: Some(format!("{}{}", self.canvas_url, endpoint)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(get_next_from_link(""), None);
        assert_eq!(get_next_from_link("<url1>; rel=\"current\",<url2>; rel=\"next\",<url3>; rel=\"first\",<url4>; rel=\"last\""), Some("url2".to_string()));
        assert_eq!(
            get_next_from_link(
                "<url1>; rel=\"current\",<url3>; rel=\"first\",<url4>; rel=\"last\""
            ),
            None
        );
    }
}
