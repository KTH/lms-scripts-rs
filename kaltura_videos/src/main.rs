extern crate dotenv;

use reqwest::blocking::Client;
use serde::Deserialize;
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

fn main() {
    dotenv::dotenv().ok();
    let entries = get_entries(50);
    println!("{:?}", entries);
}
