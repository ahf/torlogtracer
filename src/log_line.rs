extern crate time;
extern crate regex;

use std::fmt;
use std::error::Error;

use self::regex::Regex;

use tor_log::TorLogEntry;

#[derive(Debug)]
enum LogLevel {
    Debug,
    Info,
    Notice,
    Warn,
    Error,
}

impl LogLevel {
    pub fn from_string(s: String) -> LogLevel {
        match s.to_lowercase().as_ref() {
            "debug"  => LogLevel::Debug,
            "info"   => LogLevel::Info,
            "notice" => LogLevel::Notice,
            "warn"   => LogLevel::Warn,
            "error"  => LogLevel::Error,
            _        => panic!("Invalid log level {}", s),
        }
    }
}

pub struct LogLine {
    content: TorLogEntry,
    log_level: LogLevel,
    timestamp: time::Tm,
}

impl LogLine {
    pub fn new(line: String) -> LogLine {
        let expression = Regex::new(r"^(.*) \[(.*)\] (.*)$").unwrap();
        let captures = expression.captures(line.as_str()).unwrap();
        let timestamp_str = captures.get(1).unwrap().as_str();
        let log_level_str = captures.get(2).unwrap().as_str().to_string();
        let content_str   = captures.get(3).unwrap().as_str().to_string();

        match time::strptime(timestamp_str, "%b %d %H:%M:%S") {
            Err(e)        => panic!("Error parsing timestamp: {}", e.description()),
            Ok(timestamp) => {
                LogLine {
                    content:   TorLogEntry::from_string(content_str),
                    timestamp: timestamp,
                    log_level: LogLevel::from_string(log_level_str)
                }
            }
        }
    }
}

impl fmt::Display for LogLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} [{:?}] {}", self.timestamp.asctime(), self.log_level, self.content)
    }
}
