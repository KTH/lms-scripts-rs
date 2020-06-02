use csv::Writer;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

/** Structure of the JSON returned by the API */
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Todo {
    id: i32,
    user_id: i32,
    title: String,
    completed: bool,
}

/** Structure of one row to be written */
#[derive(Serialize)]
struct Row<'a> {
    id: i32,
    user_id: i32,
    title: &'a str,
    completed: bool,
}

fn main() {
    let client = Client::new();

    println!("Requesting the TODOs");
    let resp = client
        .get("https://jsonplaceholder.typicode.com/todos")
        .send()
        .expect("Error when performing the request")
        .json::<Vec<Todo>>()
        .expect("Error when parsing the JSON");

    let mut wtr = Writer::from_path("foo.csv").expect("Error when creating the writer");
    println!("Starting to write todos to `foo.csv`");
    for row in resp.iter() {
        wtr.serialize(Row {
            id: row.id,
            user_id: row.user_id,
            title: &row.title,
            completed: row.completed,
        })
        .expect("Error when writing a row");
    }
    wtr.flush().expect("Error when flushing the writer");
    println!("Finished!");
}
