# LMS Scripts, now in Rust

Collection of CLI applications and libraries for doing semi-automated tasks with Canvas LMS. Like [lms-scripts](https://github.com/kth/lms-scripts) but in Rust.

Also a collection of experiments made by a Rust beginner. You will probably see some JavaScript-developer bias :)

## Getting started

### Run the `todo_example` app:

Every app is in one directory, for example to run the [`todo_example`](./todo_example) app run:

```
cargo run -p todo_example
```

You can also run it with:

```
cd todo_example
cargo run
```

### Environmental variables and `.env` files

Some apps need environmental variables to run them (tokens, etc). It is possible to create an `.env` that will be read by `dotenv`. You can create the `.env` file per project or create one for the whole `lms-scripts-rs` workspace.

The apps will read the `.env` from the **current directory**, this being the "directory where you run the app".

- If you run this from the root directory:

  ```
  cargo run -p todo_example
  ```

  It will read `/.env` but NOT `/todo_example/.env`

- However if you run this:

  ```
  cd todo_example
  cargo run
  ```

  It will try to read `/todo_example/.env` and then `/.env`

## Development

This repository contains more than one Cargo packages (libraries and binaries) all packaged in one workspace. [Read more about Cargo workspaces in Rust book](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)

### Getting started with Rust

This repository contains two packages as examples to help people who are not very experienced in Rust: a library and an application.

Please read the `README.md` files inside those directories to get more information.

- [`todo_example`](./todo_example) is an app that reads an API and writes its data in a file with CSV format.
- [`canvas_api`](./canvas_api) is a library with helper functions to interact with the [Canvas LMS API](https://canvas.instructure.com/doc/api/). It is more or less similar to [@kth/canvas-api](https://github.com/kth/canvas-api) but in Rust.

### Create a new package

1. `cargo new --bin «name of the application»` or `cargo new --lib «name of the library»`
2. Edit [`Cargo.toml` in root](./Cargo.toml) and add the name of the package inside `members`
3. Ensure that is correct: `cargo check -p «name of the package»`
