extern crate dotenv;
mod kaltura_api;

use csv::Writer;
use kaltura_api::KalturaCategory;
use serde::Serialize;

fn write_categories(categories: impl Iterator<Item = KalturaCategory>, filename: &str) {
    let mut wtr = Writer::from_path(filename).expect("Error when creating the writer");

    #[derive(Serialize)]
    struct Row {
        course_code: String,
        count: i32,
    }

    for category in categories {
        wtr.serialize(Row {
            course_code: category.name,
            count: category.entries_count,
        })
        .expect("Error when writing a row");
    }
    wtr.flush().expect("Error when flushing the writer");
}

fn main() {
    let ks = "".to_string();

    dotenv::dotenv().ok();
    let relevant_items = kaltura_api::get_all_categories(ks)
        .filter(|item| item.name != "InContext")
        .filter(|item| item.full_name.starts_with("Canvas>site>channels"))
        .filter(|item| item.name.len() < 10)
        .filter(|item| item.entries_count > 0);

    write_categories(relevant_items, "foo.csv");
}
