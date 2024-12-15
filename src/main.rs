use regex::Regex;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn to_snake_case(name: &str) -> String {
    let re_spaces = Regex::new(r"\s+").unwrap();
    let re_caps = Regex::new(r"([A-Z])").unwrap();
    let re_underscores = Regex::new(r"_+").unwrap();

    let result = re_spaces.replace_all(name, "_");
    let result = re_caps.replace_all(&result, |caps: &regex::Captures| {
        format!("_{}", &caps[1].to_lowercase())
    });

    let result = re_underscores.replace_all(&result, "_");

    result.trim_start_matches('_').to_string()
}

fn rename_to_snake_case(path: &Path) {
    if path.is_dir() {
        // Process the directory's contents first
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    rename_to_snake_case(&entry.path());
                }
            }
        }
    }

    // Rename the current file or directory
    if let Some(file_name) = path.file_name().and_then(|name| name.to_str()) {
        let snake_case_name = to_snake_case(file_name);
        if file_name != snake_case_name {
            let new_path = path.parent().unwrap().join(snake_case_name);
            if let Err(e) = fs::rename(path, &new_path) {
                eprintln!("Failed to rename {:?} to {:?}: {}", path, new_path, e);
            }
        }
    }
}

fn main() {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    rename_to_snake_case(&current_dir);
}
