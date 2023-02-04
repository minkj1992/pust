use std::env;
use std::fs::File;
use std::io::prelude::*;

fn find(filename: &str, target: &str) -> String {
    let mut f = File::open(filename).expect(&format!("Cannot find {filename}."));
    let mut buf = String::new();
    f.read_to_string(&mut buf)
        .expect(&format!("Something went wrong reading the {filename}"));
    buf
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for '{query}' in {filename}.",);
    println!("{}", find(&filename, query));
}
