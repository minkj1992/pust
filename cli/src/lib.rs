use std::error::Error;
use std::fs;

const ARGS_NUM: usize = 2;

// infra logic
pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // 1 represents(args[0]) current program's name
        if args.len() != (ARGS_NUM + 1) {
            return Err("invalid number of args are given.");
        }

        Ok(Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
        })
    }
}

// business logic
pub fn run(c: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(c.file_path)?;
    println!("With text:\n{contents}");

    Ok(())
}
