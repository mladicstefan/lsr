use std::{fs, panic};

fn main() {
    let path = std::env::home_dir().unwrap().join("Downloads");

    let metadata = match fs::metadata(&path) {
        Ok(meta) => meta,
        Err(e) => panic!("{:?}", e),
    };
    println!("{:?}", metadata);

    let files = fs::read_dir(&path).unwrap();

    for file in files {
        let file = file.unwrap();
        let file_metadata = file.metadata().unwrap();
        println!("{:?} - {:?}", file.file_name(), file_metadata);
    }
}
