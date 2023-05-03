use std::fs::File;
use std::io;
use std::io::{BufWriter, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};

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
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Response {
    status: String,
    headers: serde_json::Value,
    body: Post,
}

fn write_json(out_dir: &Path, data: &Response) -> io::Result<()> {
    let f = File::create(out_dir)?;
    let mut writer = BufWriter::new(f);
    serde_json::to_writer_pretty(&mut writer, data)?;
    writer.flush()?;
    Ok(())
}

fn request() -> Result<Response, Box<dyn std::error::Error>> {
    let res = reqwest::blocking::get(API_URL)?;

    Ok(Response {
        status: res.status().as_str().to_owned(),
        headers: format!("{:?}", res.headers()).into(),
        body: res.json::<Post>()?,
    })
}

fn main() {
    let out_dir = Path::new(FILE_NAME);
    let res = request().unwrap();
    let _ = write_json(out_dir, &res);
}
