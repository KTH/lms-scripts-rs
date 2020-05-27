# LMS Scripts, now in Rust

## Development

This repository contains more than one Cargo packages (libraries and binaries) all packaged in one workspace.

### Run a binary

- To run one binary do `cargo run --bin «name of the package»`
- You can also `cd «name of the package»` and then `cargo run`

### Create a new binary

1. `cargo new «name of the package»`
2. Edit `Cargo.toml` and add the name of the package inside `members`
