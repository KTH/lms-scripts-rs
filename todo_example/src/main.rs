//! This is `todo_example`, a program that reads a list of TODOs from an API
//! and writes that to a CSV file.
//!
//! It uses three libraries:
//! - `reqwest` to fetch data from the API
//! - `csv` to write data to the CSV file
//! - `serde` to make the conversion between JSON string to Rust objects and
//!   from Rust objects to CSV string
use csv::Writer;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

/// Prompts to the user if they want to see a list of completed TODOs,
/// not-completed TODOs or all.
fn prompt_completed() -> Option<bool> {
    let items = vec!["Only completed", "Only non-completed", "All"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact()
        .expect("Failed to get the user input");

    match selection {
        0 => Some(true),
        1 => Some(false),
        _ => None,
    }
}

/// Prompt a file-name
fn prompt_filename() -> String {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Write a filename")
        .interact_text()
        .expect("Failed to prompt file name")
}

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

/// Fetches the TODOs from an API endpoint
fn fetch_todos(completed_filter: Option<bool>) -> Vec<Todo> {
    let client = Client::new();
    let all_todos = client
        .get("https://jsonplaceholder.typicode.com/todos")
        .send()
        .expect("Error when performing the request")
        .json::<Vec<Todo>>()
        .expect("Error when parsing the JSON");

    match completed_filter {
        None => all_todos,
        Some(is_completed) => all_todos
            .into_iter()
            .filter(|item| item.completed == is_completed)
            .collect(),
    }
}

/// Structure of the CSV row that we are going to write. Implements the
/// `Serialize` so that `serde` can convert the struct itself into a CSV string
#[derive(Serialize)]
struct Row<'a> {
    id: i32,
    user_id: i32,
    title: &'a String,
    completed: bool,
}

/// Write TODOs into a CSV file
fn write_todos(todos: Vec<Todo>, filename: String) {
    let mut wtr = Writer::from_path(filename).expect("Error when creating the writer");

    for todo in todos.iter() {
        wtr.serialize(Row {
            id: todo.id,
            user_id: todo.user_id,
            title: &todo.title,
            completed: todo.completed,
        })
        .expect("Error when writing a row");
    }
    wtr.flush().expect("Error when flushing the writer");
}

fn main() {
    let completed_filter = prompt_completed();
    let filename = prompt_filename();
    let todo_list = fetch_todos(completed_filter);
    write_todos(todo_list, filename);
}
