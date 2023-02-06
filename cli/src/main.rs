use std::env;
use std::fs::File;
use std::io::prelude::*;

struct Config {
    query: String,
    filename: String,
}

fn find(filename: &str, _: &str) -> String {
    let mut f = File::open(filename).expect(&format!("Cannot find {filename}."));
    let mut buf = String::new();

    f.read_to_string(&mut buf)
        .expect(&format!("Something went wrong reading the {filename}"));
    buf
}

fn get_config() -> Config {
    let args: Vec<String> = env::args().collect();

    Config {
        query: args[1].clone(),
        filename: args[2].clone(),
    }
}

fn main() {
    let c = get_config();

    println!("Searching for '{}' in {}.", c.query, c.filename);
    println!("{}", find(c.filename.as_str(), c.query.as_str()));
}
