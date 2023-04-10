use std::fs::{create_dir_all, set_permissions, File, Permissions};
use std::io::copy;
use std::path::Path;
use std::process::ExitCode;

use zip::ZipArchive;

fn check_args(args: &Vec<String>) -> bool {
    if args.len() == 2 {
        println!("Okay: <{}>", args[0]);
        return true;
    }
    return false;
}

fn check_comment(i: usize, comment: &str) {
    if !(comment.is_empty()) {
        println!("File {} comment: {}", i, comment);
    }
}

#[inline(always)]
fn is_dir(name: &str) -> bool {
    (*name).ends_with('/')
}

// https://doc.rust-lang.org/stable/std/process/struct.ExitCode.html#associatedconstant.SUCCESS
fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if check_args(&args) == false {
        return ExitCode::FAILURE;
    }

    let p = Path::new(&*args[1]);
    let file = File::open(p).unwrap();

    let mut archive = ZipArchive::new(file).unwrap();
    for i in 0..archive.len() {
        let mut zip_file = archive.by_index(i).unwrap();
        let out_path = match zip_file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        check_comment(i, zip_file.comment());

        if is_dir(zip_file.name()) {
            println!("File {} extracted to \"{}\"", i, out_path.display());
            create_dir_all(&out_path).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                out_path.display(),
                zip_file.size()
            );

            if let Some(parent) = out_path.parent() {
                if !parent.exists() {
                    create_dir_all(&parent).unwrap();
                }
            }

            let mut out_file = File::create(&out_path).unwrap();
            copy(&mut zip_file, &mut out_file).unwrap();
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = zip_file.unix_mode() {
                set_permissions(&out_path, Permissions::from_mode(mode)).unwrap();
            }
        }
    }
    return ExitCode::SUCCESS;
}
