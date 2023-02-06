mod log;

use log::{log, LogLevel};

fn main() {
    log(LogLevel::All, LogLevel::Minimal, "Some message");
}
