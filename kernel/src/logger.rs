extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[derive(Clone, Copy, PartialEq)]
pub enum LogLevel { Trace, Debug, Info, Warn, Error, Fatal }

pub struct LogEntry {
    pub level: LogLevel,
    pub message: String,
    pub timestamp: u64,
    pub source: String,
}

pub struct SystemLogger {
    entries: Vec<LogEntry>,
    max_entries: usize,
}

impl SystemLogger {
    pub fn new() -> Self {
        Self { entries: Vec::new(), max_entries: 1000 }
    }

    pub fn log(&mut self, level: LogLevel, message: &str, source: &str, timestamp: u64) {
        if self.entries.len() >= self.max_entries {
            self.entries.remove(0);
        }
        self.entries.push(LogEntry {
            level,
            message: String::from(message),
            timestamp,
            source: String::from(source),
        });
    }

    pub fn last_n(&self, n: usize) -> &[LogEntry] {
        let start = self.entries.len().saturating_sub(n);
        &self.entries[start..]
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn count(&self) -> usize {
        self.entries.len()
    }

    pub fn filter_by_level(&self, level: LogLevel) -> Vec<&LogEntry> {
        self.entries.iter().filter(|e| e.level == level).collect()
    }

    pub fn entries_since(&self, timestamp: u64) -> Vec<&LogEntry> {
        self.entries.iter().filter(|e| e.timestamp >= timestamp).collect()
    }
}
