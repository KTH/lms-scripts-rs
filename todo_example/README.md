# TODO example

This app reads data from https://jsonplaceholder.typicode.com/todos and creates a CSV file from that data

## Dependencies:

- [serde](https://crates.io/crates/serde). To serialize `JSON → Rust object` and deserialize `Rust object → CSV row`.
- [reqwest](https://crates.io/crates/reqwest). To perform the requests to the API
- [csv](https://crates.io/crates/csv). To write the CSV file.
- [dialoguer](https://crates.io/crates/dialoguer). To prompt things to the user
