use std::env;
use std::fs::File;
use std::io::prelude::*;

struct Config {
    query: String,
    filename: String,
}

const ARGS_NUM: usize = 2;

impl Config {
    fn new(args: &[String]) -> Config {
        // 1 represents(args[0]) current program's name
        if args.len() != (ARGS_NUM + 1) {
            panic!("not enough arguments, you should give {ARGS_NUM} args");
        }

        Config {
            query: args[1].clone(),
            filename: args[2].clone(),
        }
    }
}

fn find(filename: &str, _: &str) -> String {
    let mut f = File::open(filename).expect(&format!("Cannot find {filename}."));
    let mut buf = String::new();

    f.read_to_string(&mut buf)
        .expect(&format!("Something went wrong reading the {filename}"));
    buf
}

fn main() {
    let c: Config;
    {
        let args: Vec<String> = env::args().collect();
        c = Config::new(&args);
    }

    println!("Searching for '{}' in {}.", c.query, c.filename);
    println!("{}", find(c.filename.as_str(), c.query.as_str()));
}
