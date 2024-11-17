use chrono::prelude::*;
use std::fs;
use std::path::PathBuf;

pub fn parse_ymd_hms(date: &str, time: &str) -> DateTime<Utc> {
    let (year, month, day) = {
        let v: Vec<&str> = date.split('-').collect();
        (
            v[0].parse().unwrap(),
            v[1].parse().unwrap(),
            v[2].parse().unwrap(),
        )
    };
    let (hour, minute, second) = {
        let v: Vec<&str> = time.split(':').collect();
        (
            v[0].parse().unwrap(),
            v[1].parse().unwrap(),
            v[2].parse().unwrap(),
        )
    };
    NaiveDate::from_ymd_opt(year, month, day)
        .unwrap()
        .and_hms_opt(hour, minute, second)
        .unwrap()
        .and_utc()
}

pub fn parse_start_date_time(date: Option<String>, time: Option<String>) -> DateTime<Utc> {
    let current_date_time = Utc::now();
    let start_date = if date.is_some() && time.is_some() {
        let date = date.as_ref().unwrap();
        let time = time.as_ref().unwrap();
        parse_ymd_hms(date, time)
    } else if date.is_some() && time.is_none() {
        let date = date.as_ref().unwrap();
        let time = "00:00:00";
        parse_ymd_hms(date, time)
    } else if date.is_none() && time.is_some() {
        let date = &current_date_time.format("%Y-%m-%d").to_string();
        let time = time.as_ref().unwrap();
        parse_ymd_hms(date, time)
    } else {
        current_date_time
    };
    start_date
}

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
