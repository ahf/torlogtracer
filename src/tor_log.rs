extern crate regex;

use std::fmt;

use self::regex::{Regex, Captures};

struct TorLogEntryHandler {
    matcher: Regex,
    fun: Box<Fn(Captures) -> TorLogEntry>,
}

impl TorLogEntryHandler {
    pub fn new(r: Regex, f: Box<Fn(Captures) -> TorLogEntry>) -> TorLogEntryHandler {
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
    DescriptorFetchRequest { onion: String, blinded_key: String, directory_id: String, directory_nickname: String, directory_ip: String },
    Unknown { content: String },
}

impl TorLogEntry {
    pub fn from_string(s: String) -> TorLogEntry {
        let mut handlers = Vec::new();

        handlers.push(TorLogEntryHandler::new(Regex::new(r"Tor (.*) \((.*)\) opening new log file.").unwrap(), Box::new(|c| {
            let version     = c.get(1).unwrap().as_str().to_string();
            let git_version = c.get(2).unwrap().as_str().to_string();

            TorLogEntry::LogOpened { version, git_version }
        })));

        handlers.push(TorLogEntryHandler::new(Regex::new(r"connection_ap_handle_onion\(\): Found usable descriptor in cache for (.*). Not fetching\.\.").unwrap(), Box::new(|c| {
            let onion = c.get(1).unwrap().as_str().to_string();

            TorLogEntry::FoundUsableDescriptor { onion }
        })));

        handlers.push(TorLogEntryHandler::new(Regex::new(r"directory_launch_v3_desc_fetch\(\): Descriptor fetch request for service (.*) with blinded key (.*) to directory \$(.*)\~(.*) at (.*)").unwrap(), Box::new(|c| {
            let onion = c.get(1).unwrap().as_str().to_string();
            let blinded_key = c.get(2).unwrap().as_str().to_string();
            let directory_id = c.get(3).unwrap().as_str().to_string();
            let directory_nickname = c.get(4).unwrap().as_str().to_string();
            let directory_ip = c.get(5).unwrap().as_str().to_string();

            TorLogEntry::DescriptorFetchRequest { onion, blinded_key, directory_id, directory_nickname, directory_ip }
        })));

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
                write!(f, "LogOpened(version: {}, git: {})", version, git_version),
            &TorLogEntry::FoundUsableDescriptor { ref onion } =>
                write!(f, "FoundUsableDescriptor(onion: {}.onion)", onion),
            &TorLogEntry::DescriptorFetchRequest { ref onion, ref blinded_key, ref directory_id, ref directory_nickname, ref directory_ip } =>
                write!(f, "DescriptorFetchRequest(onion: {}.onion, blinded_key: {}, directory {{ ID: {}, nickname: {}, IP: {} }}", onion, blinded_key, directory_id, directory_nickname, directory_ip),
            &TorLogEntry::Unknown { ref content } =>
                write!(f, "Unknown({})", content),
        }
    }
}
