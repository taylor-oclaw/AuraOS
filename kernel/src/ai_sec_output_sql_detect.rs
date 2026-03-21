extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct AISecOutputSQLDetect {
    sql_statements: Vec<String>,
    detected_vulnerabilities: Vec<String>,
}

impl AISecOutputSQLDetect {
    pub fn new() -> Self {
        AISecOutputSQLDetect {
            sql_statements: Vec::new(),
            detected_vulnerabilities: Vec::new(),
        }
    }

    pub fn add_sql_statement(&mut self, statement: String) {
        self.sql_statements.push(statement);
    }

    pub fn detect_injection(&self) -> bool {
        for statement in &self.sql_statements {
            if Self::contains_vulnerable_keywords(statement) {
                return true;
            }
        }
        false
    }

    fn contains_vulnerable_keywords(statement: &str) -> bool {
        let keywords = ["SELECT", "INSERT", "UPDATE", "DELETE", "DROP"];
        for keyword in keywords.iter() {
            if statement.contains(keyword) {
                return true;
            }
        }
        false
    }

    pub fn get_detected_vulnerabilities(&self) -> Vec<String> {
        self.detected_vulnerabilities.clone()
    }

    pub fn clear_sql_statements(&mut self) {
        self.sql_statements.clear();
    }
}
