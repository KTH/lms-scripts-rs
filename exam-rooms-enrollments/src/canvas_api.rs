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

impl<'a> Iterator for PageIterator<'a> {
    type Item = Result<Response, reqwest::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        println!("Next url is {:?}", self.next_url);
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

impl CanvasApi {
    pub fn new(canvas_url: String, canvas_token: String) -> CanvasApi {
        CanvasApi {
            canvas_token,
            canvas_url,
            client: Client::new(),
        }
    }

    #[allow(dead_code)]
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
