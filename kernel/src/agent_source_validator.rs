extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn agent_source_validator_init() {
    // Initialization logic for the module
}

pub extern "C" fn agent_source_validator_exit() {
    // Cleanup logic for the module
}

pub struct AgentSourceValidator {
    sources: Vec<String>,
    valid_sources: Vec<String>,
}

impl AgentSourceValidator {
    pub fn new() -> Self {
        AgentSourceValidator {
            sources: Vec::new(),
            valid_sources: Vec::new(),
        }
    }

    pub fn add_source(&mut self, source: String) {
        self.sources.push(source);
    }

    pub fn validate_sources(&mut self) {
        for source in &self.sources {
            if self.is_valid_source(source) {
                self.valid_sources.push(source.clone());
            }
        }
    }

    fn is_valid_source(&self, source: &str) -> bool {
        // Placeholder validation logic
        source.starts_with("http://") || source.starts_with("https://")
    }

    pub fn get_valid_sources(&self) -> Vec<String> {
        self.valid_sources.clone()
    }

    pub fn clear_sources(&mut self) {
        self.sources.clear();
        self.valid_sources.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_source_validator() {
        let mut validator = AgentSourceValidator::new();
        validator.add_source(String::from("http://example.com"));
        validator.add_source(String::from("ftp://invalid.com"));
        validator.validate_sources();

        assert_eq!(validator.get_valid_sources().len(), 1);
        assert_eq!(validator.get_valid_sources()[0], "http://example.com");

        validator.clear_sources();
        assert_eq!(validator.get_valid_sources().len(), 0);
    }
}
