use std::{
    fs::File,
    io::{self, Read, Write},
    path::PathBuf,
};

use clap::Parser;

mod consts;
mod dto;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    out_dir: PathBuf,
}

fn main() {
    println!("############1. deserialize (json -> dto)############");
    let articles: Vec<dto::Article> = serde_json::from_str(consts::RAW_ARTICLES).unwrap();
    let users: Vec<dto::User> = serde_json::from_str(consts::RAW_USERS).unwrap();
    println!("{:#?}", articles);
    println!("{:#?}", users);

    println!("############2. serialize (dto -> json)############");
    let data = serialize(&articles, &users);
    println!("{:?}", data);

    println!("############3. write file############");
    let cli = Cli::parse();
    let _ = write_file(cli.out_dir.clone(), data);

    println!("############4. read json file############");
    let _ = read_file(cli.out_dir);
}

fn serialize(articles: &Vec<dto::Article>, users: &Vec<dto::User>) -> serde_json::Value {
    serde_json::json!({"users":users,"articles":articles})
}

fn write_file(out_dir: PathBuf, serialized_data: serde_json::Value) -> io::Result<()> {
    let file = File::create(out_dir)?;
    let mut writer = io::BufWriter::new(file);

    serde_json::to_writer_pretty(&mut writer, &serialized_data)?;
    writer.flush()?;
    Ok(())
}

fn read_file(s: PathBuf) -> io::Result<()> {
    let f = File::open(s).unwrap();
    let mut buf_reader = io::BufReader::new(f);

    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let json_result: serde_json::Value =
        serde_json::from_str(&contents).expect("JSON was not well-formatted");
    println!("{:#?}", json_result);
    Ok(())
}
