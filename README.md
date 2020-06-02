# LMS Scripts, now in Rust

Collection of CLI applications and libraries for doing semi-automated tasks with Canvas LMS

## Run an app

Every app is in one directory, for example the [`todo_example`](./todo_example) app. To run it:

- Copy [`.env.in`](./.env.in) to `.env` and fill it with the required environmental variables. Each application has different environmental variables required.
- Run one of the packages with `cargo run -p «name of the package»`
- You can also `cd «name of the package»` and then `cargo run`

## Development

This repository contains more than one Cargo packages (libraries and binaries) all packaged in one workspace. [Read more about Cargo workspaces in Rust book](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)

### Getting started with Rust

This repository contains two packages as examples to help developing code quickly: a library and an application.

- [`todo_example`](./todo_example) is an app that reads an API and writes its data in a file with CSV format.
- [`canvas_api`](./canvas_api) is a library with helper functions to interact with the [Canvas LMS API](https://canvas.instructure.com/doc/api/).

### Create a new package

1. `cargo new --bin «name of the application»` or `cargo new --lib «name of the library»`
2. Edit [`Cargo.toml` in root](./Cargo.toml) and add the name of the package inside `members`
3. Ensure that is correct: `cargo check -p «name of the package»`
