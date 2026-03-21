extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct AuditLog {
    entries: Vec<String>,
}

impl AuditLog {
    pub fn new() -> Self {
        AuditLog {
            entries: Vec::new(),
        }
    }

    pub fn log(&mut self, entry: &str) {
        self.entries.push(String::from(entry));
    }

    pub fn get_logs(&self) -> &[String] {
        &self.entries
    }

    pub fn clear_logs(&mut self) {
        self.entries.clear();
    }

    pub fn count_logs(&self) -> usize {
        self.entries.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_log() {
        let mut log = AuditLog::new();
        assert_eq!(log.count_logs(), 0);

        log.log("Test entry 1");
        log.log("Test entry 2");
        assert_eq!(log.count_logs(), 2);
        assert_eq!(log.get_logs()[0], "Test entry 1");
        assert_eq!(log.get_logs()[1], "Test entry 2");

        log.clear_logs();
        assert_eq!(log.count_logs(), 0);
    }
}
