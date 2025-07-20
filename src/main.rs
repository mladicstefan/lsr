use chrono::{DateTime, Local};
use colored::{Color, Colorize};
use std::{
    fs::{self, DirEntry, Metadata, ReadDir},
    os::unix::fs::PermissionsExt,
    path::Path,
    time::SystemTime,
};

fn get_extension_color(ext: &str) -> Color {
    match ext.to_lowercase().as_str() {
        // Systems Programming
        "rs" => Color::Red,
        "go" => Color::Blue,
        "c" => Color::BrightBlue,
        "cpp" | "cc" | "cxx" => Color::BrightBlue,
        "zig" => Color::BrightYellow,

        // Web Frontend
        "js" | "mjs" => Color::Yellow,
        "ts" => Color::BrightYellow,
        "jsx" | "tsx" => Color::BrightCyan,
        "html" | "htm" => Color::BrightRed,
        "css" => Color::Blue,
        "scss" | "sass" => Color::BrightMagenta,
        "vue" => Color::Green,

        // Backend/Scripting
        "py" => Color::Green,
        "rb" => Color::BrightRed,
        "php" => Color::Magenta,
        "java" => Color::BrightRed,
        "cs" => Color::BrightMagenta,
        "swift" => Color::BrightCyan,
        "kt" => Color::BrightGreen,
        "scala" => Color::Red,
        "clj" | "cljs" => Color::Green,

        // Shell/Config
        "sh" | "bash" | "zsh" => Color::BrightGreen,
        "fish" => Color::BrightBlue,
        "ps1" => Color::Blue,
        "bat" | "cmd" => Color::BrightYellow,

        // Data/Config
        "json" => Color::Yellow,
        "yaml" | "yml" => Color::BrightYellow,
        "toml" => Color::BrightCyan,
        "xml" => Color::BrightYellow,
        "csv" => Color::Green,

        // Documents
        "md" | "markdown" => Color::White,
        "txt" => Color::White,
        "pdf" => Color::BrightRed,
        "doc" | "docx" => Color::BrightBlue,

        // Images
        "png" | "jpg" | "jpeg" | "gif" | "webp" => Color::Magenta,
        "svg" => Color::BrightCyan,
        "ico" => Color::BrightMagenta,

        // Archives
        "zip" | "tar" | "gz" | "7z" | "rar" => Color::BrightRed,

        // Executables
        "exe" | "msi" => Color::Red,
        "deb" | "rpm" => Color::BrightRed,
        "dmg" => Color::BrightMagenta,

        // Default
        _ => Color::White,
    }
}

fn get_file_color(filename: &str) -> Color {
    if let Some(ext) = filename.split('.').last() {
        if ext != filename {
            // Has extension
            get_extension_color(ext)
        } else {
            Color::White // No extension
        }
    } else {
        Color::White
    }
}

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
    let file_name: String = file
        .file_name()
        .into_string()
        .expect("Failed to convert filename...");

    if metadata.file_type().is_dir() == true {
        format!("{}/", file_name.bold().blue())
    } else {
        let color = get_file_color(&file_name);
        return file_name.color(color).to_string();
    }
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
