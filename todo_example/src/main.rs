//! This is `todo_example`, a program that reads a list of TODOs from an API
//! and writes that to a CSV file.
//!
//! It uses three libraries:
//! - `reqwest` to fetch data from the API
//! - `csv` to write data to the CSV file
//! - `serde` to make the conversion between JSON string to Rust objects and
//!   from Rust objects to CSV string
use csv::Writer;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

/// Structure of the JSON returned by the API. Implements the `Deserialize`
/// so that `serde` can convert a JSON to this struct.
///
/// The `rename_all = "camelCase"` converts camelCase fields in the JSON to
/// snake_case in Rust, so "userId" in JSON becomes "user_id" in Rust
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Todo {
    id: i32,
    user_id: i32,
    title: String,
    completed: bool,
}

/// Structure of the CSV row that we are going to write. Implements the
/// `Serialize` so that `serde` can convert the struct itself into a CSV string
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
