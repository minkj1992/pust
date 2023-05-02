use std::io;
use std::io::{BufWriter, Write};
use std::path::Path;

use std::fs::File;

use serde::{Deserialize, Serialize};
use serde_json::Value;

const API_URL: &str = "https://jsonplaceholder.typicode.com/posts/1";
const FILE_NAME: &str = "output.json";

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Post {
    id: i32,
    title: String,
    body: String,
    user_id: i32,
}

fn write_json(out_dir: &Path, data: &Post) -> io::Result<()> {
    let f = File::create(out_dir)?;
    let mut writer = BufWriter::new(f);
    serde_json::to_writer_pretty(&mut writer, data)?;
    writer.flush()?;
    Ok(())
}

fn request() -> Result<Post, Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get(API_URL)?.json::<Post>()?;
    print!("{:?}", resp);
    Ok(resp)
}

fn main() {
    let out_dir = Path::new(FILE_NAME);
    let post = request().unwrap();
    let _ = write_json(out_dir, &post);
    println!("Hello, world!");
}
