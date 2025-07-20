use chrono::{DateTime, Local};
use std::{
    fs::{self, ReadDir},
    os::unix::fs::PermissionsExt,
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

fn format_datetime(modified: std::result::Result<SystemTime, std::io::Error>) -> String {
    let time = modified.expect("Failed to get system time");
    let dt: DateTime<Local> = DateTime::from(time);
    dt.format("%b %e %H:%M:%S").to_string()
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
            "{:?} - {:?} - {:?} - {:?}",
            format_permissions(perm_bits),
            format_datetime(file_metadata.modified()),
            file.file_name(),
            file_metadata.file_type(),
        );
    }
}

fn main() {
    let path = std::env::current_dir().expect("Couldn't find current dir");
    println!("Running in dir {:?}", &path);

    let metadata = fs::metadata(&path).unwrap();

    println!("{:?}", metadata);

    let files = fs::read_dir(&path).unwrap();
    get_file_metadata(files);
}
