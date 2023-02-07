use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::PathBuf,
    sync::Mutex,
    time::{SystemTime, UNIX_EPOCH},
};

use lazy_static::lazy_static;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[allow(dead_code)] // suppress complaint about "None" not being constructed
pub enum LogLevel {
    None,
    Minimal,
    All,
}

lazy_static! {
    static ref LOG_LEVEL: LogLevel = match std::env::var("LOG_LEVEL") {
        Ok(level) => match level.to_lowercase().as_ref() {
            "2" | "all" => LogLevel::All,
            "1" | "minimal" => LogLevel::Minimal,
            "0" | "none" => LogLevel::None,
            _ => LogLevel::None,
        },
        Err(_) => LogLevel::None,
    };
    static ref LOG_FILE: Mutex<File> = {
        let path = match std::env::var("LOG_FILE") {
            Ok(filename) => PathBuf::from(filename),
            Err(_) => dirs::cache_dir().unwrap().join("acme").join("log"),
        };
        Mutex::new(
            OpenOptions::new()
                .append(true)
                .create(true)
                .open(path)
                .unwrap(),
        )
    };
}

pub fn log(min_level: LogLevel, message: &str) {
    if *LOG_LEVEL >= min_level {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut log_file = LOG_FILE.lock().unwrap();

        match writeln!(log_file, "[{now}] {message}") {
            Ok(_) => (),
            Err(e) => eprintln!("{e}"),
        };
    }
}
