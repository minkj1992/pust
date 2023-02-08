use cli::Config;
use std::env;
use std::process;

// cli logic
fn main() {
    let c: Config;
    {
        let args: Vec<String> = env::args().collect();
        c = Config::build(&args).unwrap_or_else(|err| {
            eprintln!("Problem parsing args: '{err}'");
            process::exit(1);
        });
    }
    println!("Searching for '{}' in {}.", c.query, c.file_path);

    if let Err(e) = cli::run(c) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
