extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct BizClientInteractionLog {
    logs: Vec<String>,
}

impl BizClientInteractionLog {
    pub fn new() -> Self {
        BizClientInteractionLog { logs: Vec::new() }
    }

    pub fn add_log(&mut self, log_message: &str) {
        self.logs.push(log_message.to_string());
    }

    pub fn get_logs(&self) -> &[String] {
        &self.logs
    }

    pub fn clear_logs(&mut self) {
        self.logs.clear();
    }

    pub fn count_logs(&self) -> usize {
        self.logs.len()
    }

    pub fn find_log(&self, search_term: &str) -> Option<&String> {
        self.logs.iter().find(|log| log.contains(search_term))
    }
}
