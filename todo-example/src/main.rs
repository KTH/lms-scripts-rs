use csv::Writer;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Todo {
    id: i32,
    user_id: i32,
    title: String,
    completed: bool,
}

#[derive(Serialize)]
struct Todo2<'a> {
    id: i32,
    user_id: i32,
    title: &'a str,
    completed: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let resp = client
        .get("https://jsonplaceholder.typicode.com/todos")
        .send()?
        .json::<Vec<Todo>>()?;

    let mut wtr = Writer::from_path("foo.csv")?;
    for row in resp.iter() {
        wtr.serialize(Todo2 {
            id: row.id,
            user_id: row.user_id,
            title: &row.title,
            completed: row.completed,
        })?;
    }

    Ok(())
}
