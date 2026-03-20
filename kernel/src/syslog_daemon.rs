extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct SyslogDaemon {
    messages: Vec<String>,
}

impl SyslogDaemon {
    pub fn new() -> Self {
        SyslogDaemon {
            messages: Vec::new(),
        }
    }

    pub fn log(&mut self, message: &str) {
        self.messages.push(String::from(message));
    }

    pub fn get_logs(&self) -> &[String] {
        &self.messages
    }

    pub fn clear_logs(&mut self) {
        self.messages.clear();
    }

    pub fn count_logs(&self) -> usize {
        self.messages.len()
    }

    pub fn find_log(&self, query: &str) -> Option<&String> {
        self.messages.iter().find(|msg| msg.contains(query))
    }
}
