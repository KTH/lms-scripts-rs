use csv::Writer;
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://jsonplaceholder.typicode.com/todos")
        .await?
        .json::<Vec<Todo>>()
        .await?;

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
