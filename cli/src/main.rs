use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::process;

const ARGS_NUM: usize = 2;

// infra logic
struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        // 1 represents(args[0]) current program's name
        if args.len() != (ARGS_NUM + 1) {
            return Err("invalid number of args are given.");
        }

        Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone(),
        })
    }
}

// business logic
fn run(c: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(c.filename)?;

    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    println!("With text:\n{}", buf);
    Ok(())
}

// cli logic
fn main() {
    let c: Config;
    {
        let args: Vec<String> = env::args().collect();
        c = Config::build(&args).unwrap_or_else(|err| {
            println!("Problem parsing args: '{err}'");
            process::exit(1);
        });
    }

    println!("Searching for '{}' in {}.", c.query, c.filename);

    if let Err(e) = run(c) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
