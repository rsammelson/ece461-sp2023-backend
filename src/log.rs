use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[allow(dead_code)] // suppress complaint about "None" not being constructed
pub enum LogLevel {
    None,
    Minimal,
    All,
}

pub fn log(cur_level: LogLevel, min_level: LogLevel, message: &str) {
    if cur_level >= min_level {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        eprintln!("[{now}] {message}");
    }
}
