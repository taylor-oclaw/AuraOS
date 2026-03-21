extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    let validator = AISecOutputValidator::new();
    validator.validate_output("valid output");
    validator.log_error("An error occurred");
    validator.add_whitelist_entry("trusted_source");
    validator.remove_whitelist_entry("untrusted_source");
    validator.is_whitelisted("trusted_source");
}

pub struct AISecOutputValidator {
    whitelist: Vec<String>,
    errors: Vec<String>,
}

impl AISecOutputValidator {
    pub fn new() -> Self {
        AISecOutputValidator {
            whitelist: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn validate_output(&mut self, output: &str) {
        if self.is_whitelisted(output) {
            println!("Output is valid and whitelisted.");
        } else {
            self.log_error("Output is not whitelisted.");
        }
    }

    pub fn log_error(&mut self, error: &str) {
        self.errors.push(String::from(error));
        println!("Error logged: {}", error);
    }

    pub fn add_whitelist_entry(&mut self, entry: &str) {
        if !self.is_whitelisted(entry) {
            self.whitelist.push(String::from(entry));
            println!("Whitelist entry added: {}", entry);
        } else {
            println!("Entry already in whitelist.");
        }
    }

    pub fn remove_whitelist_entry(&mut self, entry: &str) {
        if let Some(index) = self.whitelist.iter().position(|e| e == entry) {
            self.whitelist.remove(index);
            println!("Whitelist entry removed: {}", entry);
        } else {
            println!("Entry not found in whitelist.");
        }
    }

    pub fn is_whitelisted(&self, entry: &str) -> bool {
        self.whitelist.contains(&String::from(entry))
    }
}
