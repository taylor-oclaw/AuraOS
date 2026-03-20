extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn ai_output_validator_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn ai_output_validator_exit() {
    // Cleanup logic for the module
}

pub struct AIOutputValidator {
    valid_outputs: Vec<String>,
    invalid_outputs: Vec<String>,
}

impl AIOutputValidator {
    pub fn new() -> Self {
        AIOutputValidator {
            valid_outputs: Vec::new(),
            invalid_outputs: Vec::new(),
        }
    }

    pub fn add_valid_output(&mut self, output: String) {
        self.valid_outputs.push(output);
    }

    pub fn add_invalid_output(&mut self, output: String) {
        self.invalid_outputs.push(output);
    }

    pub fn is_output_valid(&self, output: &str) -> bool {
        self.valid_outputs.contains(&String::from(output))
    }

    pub fn list_valid_outputs(&self) -> Vec<String> {
        self.valid_outputs.clone()
    }

    pub fn list_invalid_outputs(&self) -> Vec<String> {
        self.invalid_outputs.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_output_validator() {
        let mut validator = AIOutputValidator::new();
        validator.add_valid_output(String::from("valid1"));
        validator.add_invalid_output(String::from("invalid1"));

        assert!(validator.is_output_valid("valid1"));
        assert!(!validator.is_output_valid("invalid1"));
        assert!(!validator.is_output_valid("unknown"));

        let valid_outputs = validator.list_valid_outputs();
        assert_eq!(valid_outputs, vec![String::from("valid1")]);

        let invalid_outputs = validator.list_invalid_outputs();
        assert_eq!(invalid_outputs, vec![String::from("invalid1")]);
    }
}
