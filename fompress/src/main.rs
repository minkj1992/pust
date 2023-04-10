use std::fs::File;
use std::io::{copy, BufReader};
use std::os::unix::prelude::MetadataExt;
use std::path::{Path, PathBuf};
use std::time::Instant;

use clap::Parser;
use flate2::write::{GzEncoder, ZlibEncoder};
use flate2::Compression;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    source: PathBuf,
}

#[inline(always)]
fn read_file(s: PathBuf) -> BufReader<File> {
    BufReader::new(File::open(s).unwrap())
}

fn zlib_compress_write_file(mut r: BufReader<File>, w: File) -> File {
    let mut enc = ZlibEncoder::new(w, Compression::default());
    copy(&mut r, &mut enc).unwrap();
    enc.finish().unwrap()
}

fn gz_compress_write_file(mut r: BufReader<File>, w: File) -> File {
    let mut enc = GzEncoder::new(w, Compression::default());
    copy(&mut r, &mut enc).unwrap();
    enc.finish().unwrap()
}

fn main() {
    let cli = Cli::parse();
    let src_file_name = cli.source.file_stem().unwrap().to_str().unwrap();
    let zlib_output = File::create(Path::new(&format!("zlib_{}", src_file_name))).unwrap();
    let gz_output = File::create(Path::new(&format!("gz_{}", src_file_name))).unwrap();

    // 1. Zlib compress
    let start = Instant::now();
    let output = zlib_compress_write_file(read_file(cli.source.clone()), zlib_output);
    println!("Zlib Elapsed : {:?}", start.elapsed());
    println!("Zlib target size: {:?}", output.metadata().unwrap().size());

    // 2. Gz compress
    let start = Instant::now();
    let output = gz_compress_write_file(read_file(cli.source.clone()), gz_output);
    println!("Gz Elapsed : {:?}", start.elapsed());
    println!("Gz target size: {:?}", output.metadata().unwrap().size());
}
