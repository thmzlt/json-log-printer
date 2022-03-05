use chrono::{DateTime, Local};
use owo_colors::{OwoColorize, Style};
use serde::Deserialize;
use serde_json::{Result, Value};
use std::fmt;
use std::io;

#[derive(Deserialize)]
struct LogRecord {
    time: LogTime,
    level: LogLevel,
    message: String,
    extra: Value,
}

#[derive(Deserialize)]
struct LogTime(DateTime<Local>);

#[derive(Copy, Clone, Deserialize)]
enum LogLevel {
    DEBUG,
    INFO,
    WARNING,
    ERROR,
}

#[derive(Deserialize)]
struct LogMessage(LogLevel, String);

impl fmt::Display for LogRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.time,
            LogMessage(self.level, self.message.clone()),
            format!("{}", self.extra).style(Style::new().white().dimmed()),
        )
    }
}

impl fmt::Display for LogTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = self.0.format("%H:%M:%S");

        write!(f, "{}", value.style(Style::new().cyan()))
    }
}

impl fmt::Display for LogMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let style = match self.0 {
            LogLevel::DEBUG => Style::new().blue(),
            LogLevel::INFO => Style::new().white(),
            LogLevel::WARNING => Style::new().yellow().bold(),
            LogLevel::ERROR => Style::new().red().bold(),
        };
        let message_with_level = format!("{}", self.1);

        write!(f, "{}", message_with_level.style(style))
    }
}

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();

    while stdin.read_line(&mut buffer).is_ok() {
        let line = buffer.trim_end();
        let record: Result<LogRecord> = serde_json::from_str(line);

        match record {
            Ok(record) => println!("{}", record),
            Err(_) => {
                panic!("Couldn't parse log message");
            }
        }

        buffer.clear();
    }
}
