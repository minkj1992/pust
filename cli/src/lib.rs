use impl_tools::impl_scope;
use std::env;
use std::error::Error;
use std::fs;

impl_scope! {
    #[impl_default]
    pub struct Config {
        pub query: String,
        pub file_path: String,
        pub ignore_case:bool=false,
    }
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if let Err(e) = Self::validate(args) {
            return Err(e);
        }
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
            ignore_case,
        })
    }

    fn validate(args: &[String]) -> Result<(), &'static str> {
        let args_num: usize = 3;
        let n = args.len() - 1;

        // 1 represents(args[0]) current program's name
        if n > args_num {
            return Err("over arguments");
        }
        Ok(())
    }
}

// business logic
pub fn run(c: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(c.file_path)?;
    let lines = if c.ignore_case {
        search_case_insensitive(&c.query, &contents)
    } else {
        search(&c.query, &contents)
    };

    for line in lines {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config() {
        let c = Config {
            ..Default::default()
        };

        assert_eq!(c.file_path, "");
        assert_eq!(c.query, "");
        assert_eq!(c.ignore_case, false);
    }

    #[test]
    fn with_capital() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
