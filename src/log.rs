use std::time::{SystemTime, UNIX_EPOCH};

use lazy_static::lazy_static;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[allow(dead_code)] // suppress complaint about "None" not being constructed
pub enum LogLevel {
    None,
    Minimal,
    All,
}

lazy_static! {
    static ref LOG_LEVEL: LogLevel = match std::env::var("acme_log_level") {
        Ok(level) => match level.to_lowercase().as_ref() {
            "all" => LogLevel::All,
            "minimal" => LogLevel::Minimal,
            "none" => LogLevel::None,
            _ => LogLevel::None,
        },
        Err(_) => LogLevel::None,
    };
}

pub fn log(min_level: LogLevel, message: &str) {
    if *LOG_LEVEL >= min_level {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        eprintln!("[{now}] {message}");
    }
}
