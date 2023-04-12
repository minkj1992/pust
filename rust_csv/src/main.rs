use std::{error::Error, process::ExitCode};

use csv;

mod config;

fn read_from_file(p: &str) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(p)?;

    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() -> ExitCode {
    let cfg = config::ConfigBuilder::new().build();
    let src = cfg.src.to_str().unwrap();

    if let Err(err) = read_from_file(&src) {
        eprintln!("{}", err);
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
