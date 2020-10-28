use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KalturaCategory {
    pub name: String,
    pub full_name: String,
    pub entries_count: i32,
}

/// Returns an Iterator that goes through all Kaltura categories. Automatically
/// fetches the following pages until the end.
pub fn get_all_categories(ks: String) -> impl Iterator<Item = KalturaCategory> {
    struct KalturaIterator {
        ks: String,
        page: i32,
        items: Vec<KalturaCategory>,
    }

    impl Iterator for KalturaIterator {
        type Item = KalturaCategory;

        fn next(&mut self) -> Option<Self::Item> {
            let item = self.items.pop();

            match item {
                Some(item) => Some(item),
                None => {
                    self.page += 1;

                    match fetch_categories(&self.ks, self.page) {
                        None => None,
                        Some(new_categories) => {
                            self.items = new_categories;
                            self.items.pop()
                        }
                    }
                }
            }
        }
    }

    KalturaIterator {
        ks,
        page: 0,
        items: vec![],
    }
}

/// Fetch all Kaltura Categories in one page
fn fetch_categories(ks: &String, page: i32) -> Option<Vec<KalturaCategory>> {
    let request_body = format!(
        r#"{{"ks":"{}","responseProfile": {{"objectType": "KalturaDetachedResponseProfile","type":1,"fields":"id, name, createdAt, directSubCategoriesCount, entriesCount, fullName, tags, parentId, privacyContexts"}},"filter":{{"objectType":"KalturaCategoryFilter","orderBy":"-createdAt","advancedSearch": {{"objectType":"KalturaSearchOperator","type":1,"items":[{{"objectType":"KalturaMetadataSearchItem","type":1,"metadataProfileId":2001}}]}}}},"pager":{{"objectType":"KalturaFilterPager","pageSize":250,"pageIndex":{}}},"apiVersion":"15.6.0"}}"#,
        ks, page
    );

    let client = Client::new();
    let response = client
        .post("https://api.kaltura.nordu.net/api_v3/service/category/action/list?format=1&clientTag=kmcng")
        .header("Content-Type", "application/json")
        .body(request_body)
        .send()
        .unwrap();

    #[derive(Deserialize, Debug)]
    struct KalturaResponse {
        objects: Vec<KalturaCategory>,
    }

    match response.json::<KalturaResponse>() {
        Ok(body) => Some(body.objects),
        _ => None,
    }
}
