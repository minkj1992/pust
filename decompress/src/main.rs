use std::fs;
use std::io::copy;
use std::path::PathBuf;
use std::process::ExitCode;

use zip::ZipArchive;

struct Config {
    src: PathBuf,
    dest: PathBuf,
}

impl Config {
    const VALID_LEN: [usize; 2] = [2, 3];

    fn new(args: &Vec<String>) -> Result<Config, &str> {
        if !Config::is_valid(&args) {
            return Err("Must pass 3 arguments, source dir and target dir");
        }
        let src = PathBuf::from(&args[1].clone());

        let dest = match args.len() {
            2 => PathBuf::from(src.clone().file_stem().unwrap()),
            _ => PathBuf::from(&args[2].clone()),
        };

        Ok(Config { src, dest })
    }

    #[inline]
    fn is_valid(args: &Vec<String>) -> bool {
        return if Config::VALID_LEN.contains(&args.len()) {
            true
        } else {
            false
        };
    }
}

fn decompress(from: PathBuf, to: PathBuf) {
    fn check_comment(i: usize, comment: &str) {
        if !(comment.is_empty()) {
            println!("File {} comment: {}", i, comment);
        }
    }

    #[inline]
    fn is_dir(name: &str) -> bool {
        (*name).ends_with('/')
    }

    fn set_permissions(mode: u32, out_path: &PathBuf) {
        #[cfg(unix)]
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(out_path, fs::Permissions::from_mode(mode)).unwrap();
    }

    let source = fs::File::open(from).unwrap();
    let mut archive = ZipArchive::new(source).unwrap();
    for i in 0..archive.len() {
        let mut zip_file = archive.by_index(i).unwrap();
        let out_path = match zip_file.enclosed_name() {
            Some(path) => to.as_path().join(path).to_owned(),
            None => continue,
        };

        check_comment(i, zip_file.comment());

        if is_dir(zip_file.name()) {
            println!("File {} extracted to \"{}\"", i, out_path.display());
            fs::create_dir_all(&out_path).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                out_path.display(),
                zip_file.size()
            );

            if let Some(parent) = out_path.parent() {
                if !parent.exists() {
                    fs::create_dir_all(&parent).unwrap();
                }
            }

            let mut out_file = fs::File::create(&out_path).unwrap();
            copy(&mut zip_file, &mut out_file).unwrap();
        }

        if let Some(mode) = zip_file.unix_mode() {
            set_permissions(mode, &out_path);
        }
    }
}

// https://doc.rust-lang.org/stable/std/process/struct.ExitCode.html#associatedconstant.SUCCESS
fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let cfg = match Config::new(&args) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        }
    };

    fs::create_dir_all(&cfg.dest).unwrap();
    decompress(cfg.src, cfg.dest);
    return ExitCode::SUCCESS;
}
