use std::fs::File;
use std::io::{copy, BufReader};
use std::path::PathBuf;
use std::time::Instant;

use clap::Parser;
use flate2::write::ZlibEncoder;
use flate2::Compression;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    source: PathBuf,
    #[arg(short, long)]
    target: PathBuf,
}

#[inline(always)]
fn read_file(s: PathBuf) -> BufReader<File> {
    BufReader::new(File::open(s).unwrap())
}

fn compress_write_file(mut r: BufReader<File>, w: File) -> File {
    let mut enc = ZlibEncoder::new(w, Compression::default());
    copy(&mut r, &mut enc).unwrap();
    enc.finish().unwrap()
}

fn main() {
    let cli = Cli::parse();

    let start = Instant::now();
    let output = compress_write_file(read_file(cli.source), File::create(cli.target).unwrap());
    println!("Elapsed: {:?}", start.elapsed());

    println!("Target metadata: {:?}", output.metadata().unwrap());
}
