use std::fs::{self, ReadDir};

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
        println!(
            "{:?} - {:?} - {:?} - {:?}",
            file.file_name(),
            file_metadata.permissions(),
            file_metadata.modified(),
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
