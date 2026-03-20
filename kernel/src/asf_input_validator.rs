extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn asf_input_validator_init() {
    // Initialization code if needed
}

#[no_mangle]
pub extern "C" fn asf_input_validator_exit() {
    // Cleanup code if needed
}

pub struct InputValidator {
    allowed_chars: Vec<char>,
    max_length: usize,
}

impl InputValidator {
    pub fn new(allowed_chars: &[char], max_length: usize) -> Self {
        InputValidator {
            allowed_chars: allowed_chars.to_vec(),
            max_length,
        }
    }

    pub fn is_valid(&self, input: &str) -> bool {
        if input.len() > self.max_length {
            return false;
        }
        for c in input.chars() {
            if !self.allowed_chars.contains(&c) {
                return false;
            }
        }
        true
    }

    pub fn add_allowed_char(&mut self, c: char) {
        if !self.allowed_chars.contains(&c) {
            self.allowed_chars.push(c);
        }
    }

    pub fn remove_allowed_char(&mut self, c: char) {
        self.allowed_chars.retain(|&x| x != c);
    }

    pub fn set_max_length(&mut self, max_length: usize) {
        self.max_length = max_length;
    }

    pub fn get_allowed_chars(&self) -> Vec<char> {
        self.allowed_chars.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_validator() {
        let mut validator = InputValidator::new(&['a', 'b', 'c'], 5);
        assert!(validator.is_valid("abc"));
        assert!(!validator.is_valid("abcd"));
        assert!(!validator.is_valid("ab1"));

        validator.add_allowed_char('1');
        assert!(validator.is_valid("ab1"));

        validator.remove_allowed_char('a');
        assert!(!validator.is_valid("ab1"));

        validator.set_max_length(6);
        assert!(validator.is_valid("abcdef"));
    }
}
