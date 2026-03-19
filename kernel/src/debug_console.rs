extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

pub struct LogEntry {
    pub timestamp: u64,
    pub level: LogLevel,
    pub module: String,
    pub message: String,
}

pub struct DebugConsole {
    pub entries: Vec<LogEntry>,
    pub max_entries: usize,
    pub min_level: u8,
    pub paused: bool,
    pub next_id: u64,
}

impl DebugConsole {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            max_entries: 10000,
            min_level: 2,
            paused: false,
            next_id: 1,
        }
    }

    pub fn log(&mut self, level: LogLevel, module: &str, msg: &str) {
        if self.paused {
            return;
        }
        self.entries.push(LogEntry {
            timestamp: 0,
            level,
            module: String::from(module),
            message: String::from(msg),
        });
        if self.entries.len() > self.max_entries {
            self.entries.remove(0);
        }
    }

    pub fn info(&mut self, module: &str, msg: &str) {
        self.log(LogLevel::Info, module, msg);
    }

    pub fn error(&mut self, module: &str, msg: &str) {
        self.log(LogLevel::Error, module, msg);
    }

    pub fn warn(&mut self, module: &str, msg: &str) {
        self.log(LogLevel::Warn, module, msg);
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn errors_only(&self) -> Vec<&LogEntry> {
        self.entries
            .iter()
            .filter(|e| matches!(e.level, LogLevel::Error | LogLevel::Fatal))
            .collect()
    }

    pub fn by_module(&self, module: &str) -> Vec<&LogEntry> {
        self.entries.iter().filter(|e| e.module == module).collect()
    }

    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }
}
