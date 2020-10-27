extern crate dotenv;

use csv::Writer;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct KalturaCategory {
    name: String,
    full_name: String,
    entries_count: i32,
}

#[derive(Deserialize, Debug)]
struct KalturaResponse {
    objects: Vec<KalturaCategory>,
}

#[derive(Serialize)]
struct Output {
    course_code: String,
    count: i32,
}

fn get_entries(page: i32) -> Option<Vec<KalturaCategory>> {
    let request_body = format!(
        r#"{{"ks":"{}","responseProfile": {{"objectType": "KalturaDetachedResponseProfile","type":1,"fields":"id, name, createdAt, directSubCategoriesCount, entriesCount, fullName, tags, parentId, privacyContexts"}},"filter":{{"objectType":"KalturaCategoryFilter","orderBy":"-createdAt","advancedSearch": {{"objectType":"KalturaSearchOperator","type":1,"items":[{{"objectType":"KalturaMetadataSearchItem","type":1,"metadataProfileId":2001}}]}}}},"pager":{{"objectType":"KalturaFilterPager","pageSize":250,"pageIndex":{}}},"apiVersion":"15.6.0"}}"#,
        env::var("KALTURA_KS").unwrap(),
        page
    );

    let client = Client::new();
    let response = client
        .post("https://api.kaltura.nordu.net/api_v3/service/category/action/list?format=1&clientTag=kmcng")
        .header("Content-Type", "application/json")
        .body(request_body)
        .send()
        .unwrap();

    match response.json::<KalturaResponse>() {
        Ok(body) => Some(body.objects),
        _ => None,
    }
}

struct PageIterator {
    current_page: i32,
}

impl Iterator for PageIterator {
    type Item = Vec<KalturaCategory>;

    fn next(&mut self) -> Option<Self::Item> {
        self.current_page += 1;
        get_entries(self.current_page)
    }
}

struct ItemIterator {
    page_iterator: PageIterator,
    items: Vec<KalturaCategory>,
}

impl Iterator for ItemIterator {
    type Item = KalturaCategory;

    fn next(&mut self) -> Option<Self::Item> {
        let element = self.items.pop();

        match element {
            Some(item) => Some(item),
            None => match self.page_iterator.next() {
                None => None,
                Some(page) => {
                    self.items = page;
                    self.items.pop()
                }
            },
        }
    }
}

fn get_all_entries() -> ItemIterator {
    ItemIterator {
        page_iterator: PageIterator { current_page: 0 },
        items: vec![],
    }
}

fn main() {
    let mut wtr = Writer::from_path("kaltura_list.csv").unwrap();

    dotenv::dotenv().ok();
    let relevant_items = get_all_entries()
        .filter(|item| item.name != "InContext")
        .filter(|item| item.full_name.starts_with("Canvas>site>channels"))
        .filter(|item| item.name.len() < 10)
        .filter(|item| item.entries_count > 0);

    for item in relevant_items {
        wtr.serialize(Output {
            course_code: item.name,
            count: item.entries_count,
        })
        .unwrap();
    }
}
