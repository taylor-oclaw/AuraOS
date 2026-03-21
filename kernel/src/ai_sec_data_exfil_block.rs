extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct AiSecDataExfilBlock {
    blocked_data: Vec<String>,
    allowed_data: Vec<String>,
    log: Vec<String>,
}

impl AiSecDataExfilBlock {
    pub fn new() -> Self {
        AiSecDataExfilBlock {
            blocked_data: Vec::new(),
            allowed_data: Vec::new(),
            log: Vec::new(),
        }
    }

    pub fn block_data(&mut self, data: &str) {
        let data_str = String::from(data);
        if !self.blocked_data.contains(&data_str) {
            self.blocked_data.push(data_str.clone());
            self.log_event(String::from("info"));
        }
    }

    pub fn allow_data(&mut self, data: &str) {
        let data_str = String::from(data);
        if !self.allowed_data.contains(&data_str) {
            self.allowed_data.push(data_str.clone());
            self.log_event(String::from("info"));
        }
    }

    pub fn is_blocked(&self, data: &str) -> bool {
        self.blocked_data.contains(data)
    }

    pub fn is_allowed(&self, data: &str) -> bool {
        self.allowed_data.contains(data)
    }

    pub fn log_event(&mut self, event: String) {
        self.log.push(event);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_and_allow() {
        let mut module = AiSecDataExfilBlock::new();
        module.block_data("sensitive");
        assert!(module.is_blocked("sensitive"));
        module.allow_data("public");
        assert!(module.is_allowed("public"));
    }

    #[test]
    fn test_logging() {
        let mut module = AiSecDataExfilBlock::new();
        module.log_event(String::from("Test log entry"));
        assert_eq!(module.log.len(), 1);
    }
}
