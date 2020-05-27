extern crate dotenv;

use dotenv::dotenv;
use std::env;

fn env(key: &str) -> String {
    match env::var(key) {
        Ok(val) => val,
        Err(_) => panic!("Environmental variable {} not defined", key)
    }
}

fn main() {
    dotenv().ok();
    let m = env("MEANING_XOF_LIFE");

    println!("Meaning {}", m);
}
