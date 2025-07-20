use chrono::{DateTime, Local};
use colored::Colorize;
use std::{
    fs::{self, DirEntry, Metadata, ReadDir},
    os::unix::fs::PermissionsExt,
    path::Path,
    time::SystemTime,
};

fn format_permissions(mode: u32) -> String {
    // remove filetype bits
    let perm_bits = mode & 0o777;
    let mut result = String::with_capacity(10);

    // shift the bitmask for all 3 octal pairs in binary
    for shift in [6, 3, 0] {
        let group = (perm_bits >> shift) & 0o7;

        //check read bit
        result.push(if group & 0o4 != 0 { 'r' } else { '-' });
        //check write bit
        result.push(if group & 0o2 != 0 { 'w' } else { '-' });
        // check execute bit
        result.push(if group & 0o1 != 0 { 'x' } else { '-' });
    }
    return result;
}

fn format_datetime(modified: &std::result::Result<SystemTime, std::io::Error>) -> String {
    let time = modified.as_ref().expect("Failed to get system time");
    let dt: DateTime<Local> = DateTime::from(*time);
    return dt.format("%b %e %H:%M:%S").to_string();
}

fn format_name(file: &DirEntry, metadata: &Metadata) -> String {
    let mut file_name: String = file
        .file_name()
        .into_string()
        .expect("Failed to convert filename...");

    if metadata.file_type().is_dir() == true {
        file_name.push_str("/.");
        file_name = file_name.bold().blue().to_string();
    }
    return file_name;
}

fn get_file_metadata(files: ReadDir) {
    for file in files {
        let file = match file {
            Ok(f) => f,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        };

        let file_metadata = match file.metadata() {
            Ok(meta) => meta,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        };

        let perm_bits: u32 = file_metadata.permissions().mode();

        println!(
            "{} - {} - {}",
            format_permissions(perm_bits),
            format_datetime(&file_metadata.modified()),
            format_name(&file, &file_metadata),
            // file_metadata.file_type(),
        );
    }
}

fn get_pwd_metadata(path: &Path) {
    let metadata = match fs::metadata(&path) {
        Ok(meta) => meta,
        Err(e) => {
            eprint!("{:?}", e);
            return;
        }
    };
    let str_path = path.to_str().expect("Failed to convert PWD to str");

    let perm_bits: u32 = metadata.permissions().mode();
    println!(
        "{} - {}",
        format_permissions(perm_bits),
        str_path.bold().blue().to_string(),
    );
}

fn main() {
    let path = match std::env::current_dir() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Error: {:?}", e);
            std::process::exit(1);
        }
    };
    get_pwd_metadata(&path);
    let files = fs::read_dir(&path).unwrap();
    get_file_metadata(files);
}
