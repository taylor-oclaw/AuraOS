extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct AuraRecoveryMode {
    status: String,
    log_entries: Vec<String>,
}

impl AuraRecoveryMode {
    pub fn new() -> Self {
        AuraRecoveryMode {
            status: "Initialized".to_string(),
            log_entries: Vec::new(),
        }
    }

    pub fn start_recovery(&mut self) {
        self.status = "Recovery Started".to_string();
        self.log("Recovery process initiated.");
    }

    pub fn is_recovering(&self) -> bool {
        self.status == "Recovery Started"
    }

    pub fn log(&mut self, message: &str) {
        let entry = format!("Log: {}", message);
        self.log_entries.push(entry);
    }

    pub fn get_log_entries(&self) -> &[String] {
        &self.log_entries
    }

    pub fn complete_recovery(&mut self) {
        if self.is_recovering() {
            self.status = "Recovery Completed".to_string();
            self.log("Recovery process completed successfully.");
        } else {
            self.log("No recovery process to complete.");
        }
    }
}
