extern crate regex;

use std::fmt;

use self::regex::{Regex, Captures};

/*
fn foo<T>(f: T) -> i32 
  where T : Fn(String) -> i32 {
    f(String::from("foo"))
}
*/

struct TorLogEntryHandler<F> {
    matcher: Regex,
    fun: F,
}

impl<F> TorLogEntryHandler<F> where F : Fn(Captures) -> TorLogEntry {
    pub fn new(r: Regex, f: F) -> TorLogEntryHandler<F> {
        TorLogEntryHandler {
            matcher: r,
            fun: f,
        }
    }

    pub fn handle(&self, s: &String) -> Option<TorLogEntry> {
        if self.matcher.is_match(s.as_str()) {
            let captures = self.matcher.captures(s.as_str()).unwrap();
            Some((self.fun)(captures))
        } else {
            None
        }
    }
}

pub enum TorLogEntry {
    LogOpened { version: String, git_version: String },
    FoundUsableDescriptor { onion: String },
    Unknown   { content: String },
}

impl TorLogEntry {
    pub fn from_string(s: String) -> TorLogEntry {
        let mut handlers = Vec::new();

        handlers.push(TorLogEntryHandler::new(Regex::new(r"Tor (.*) \((.*)\) opening new log file.").unwrap(), |a| {
            let version     = a.get(1).unwrap().as_str().to_string();
            let git_version = a.get(2).unwrap().as_str().to_string();

            TorLogEntry::LogOpened { version, git_version }
        }));

        handlers.push(TorLogEntryHandler::new(Regex::new(r"connection_ap_handle_onion\(\): Found usable descriptor in cache for (.*)\.onion. Not fetching\.\.").unwrap(), |b| {
            let onion = b.get(1).unwrap().as_str().to_string();

            TorLogEntry::FoundUsableDescriptor { onion }
        }));

        for handler in handlers {
            match handler.handle(&s) {
                Some(r) => return r,
                None    => continue,
            }
        }

        TorLogEntry::Unknown { content: s }
    }
}

impl fmt::Display for TorLogEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &TorLogEntry::LogOpened { ref version, ref git_version } =>
                write!(f, "LogOpened({}, {})", version, git_version),
            &TorLogEntry::Unknown { ref content } =>
                write!(f, "Unknown({})", content),
        }
    }
}
