use std::fs;
use std::path::PathBuf;

fn main() {
    if PathBuf::from("types.toml").is_file() {
        fs::remove_file(PathBuf::from("types.toml")).unwrap();
    }
}