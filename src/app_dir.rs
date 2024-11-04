use std::fs;
use std::path::PathBuf;

fn get_app_dir() -> PathBuf {
    let mut path = match home::home_dir() {
        Some(path) => path,
        None => {
            println!("Could not find home directory.");
            return PathBuf::new();
        }
    };
    path.push(".todo");
    if !path.exists() {
        match fs::create_dir(&path) {
            Ok(_) => (),
            Err(e) => {
                println!("Could not create directory: {:?}", e);
                return PathBuf::new();
            }
        }
    }
    path
}

/// Get the path to the todo database (sqlite).
pub fn get_app_db_path() -> PathBuf {
    let mut path = get_app_dir();
    path.push("todo.db");
    path
}
