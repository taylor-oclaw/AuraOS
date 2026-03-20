extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

mod audit_log {
    use super::*;

    pub struct AuditLog {
        entries: Vec<String>,
    }

    impl AuditLog {
        pub fn new() -> Self {
            AuditLog {
                entries: Vec::new(),
            }
        }

        pub fn log(&mut self, message: &str) {
            self.entries.push(String::from(message));
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

        pub fn find_log(&self, message: &str) -> Option<&String> {
            self.entries.iter().find(|&entry| entry == message)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::audit_log::*;

    #[test]
    fn test_audit_log() {
        let mut log = AuditLog::new();
        assert_eq!(log.count_logs(), 0);

        log.log("User logged in");
        log.log("File accessed");

        assert_eq!(log.count_logs(), 2);
        assert_eq!(log.get_logs().len(), 2);

        assert!(log.find_log("User logged in").is_some());
        assert!(log.find_log("File accessed").is_some());

        log.clear_logs();
        assert_eq!(log.count_logs(), 0);
    }
}
