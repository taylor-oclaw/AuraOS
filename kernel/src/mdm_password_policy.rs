extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

mod mdm_password_policy {
    use super::*;

    pub struct PasswordPolicy {
        min_length: usize,
        max_length: usize,
        require_uppercase: bool,
        require_lowercase: bool,
        require_digit: bool,
        require_special_char: bool,
    }

    impl PasswordPolicy {
        pub fn new(
            min_length: usize,
            max_length: usize,
            require_uppercase: bool,
            require_lowercase: bool,
            require_digit: bool,
            require_special_char: bool,
         -> Self {
            PasswordPolicy {
                min_length,
                max_length,
                require_uppercase,
                require_lowercase,
                require_digit,
                require_special_char,
            }
        }

        pub fn is_valid(&self, password: &str) -> bool {
            if password.len() < self.min_length || password.len() > self.max_length {
                return false;
            }

            let mut has_upper = false;
            let mut has_lower = false;
            let mut has_digit = false;
            let mut has_special = false;

            for c in password.chars() {
                if c.is_uppercase() {
                    has_upper = true;
                } else if c.is_lowercase() {
                    has_lower = true;
                } else if c.is_digit(10) {
                    has_digit = true;
                } else if !c.is_alphanumeric() {
                    has_special = true;
                }
            }

            (self.require_uppercase && has_upper)
                && (self.require_lowercase && has_lower)
                && (self.require_digit && has_digit)
                && (self.require_special_char && has_special)
        }

        pub fn get_min_length(&self) -> usize {
            self.min_length
        }

        pub fn set_min_length(&mut self, min_length: usize) {
            self.min_length = min_length;
        }

        pub fn get_max_length(&self) -> usize {
            self.max_length
        }

        pub fn set_max_length(&mut self, max_length: usize) {
            self.max_length = max_length;
        }

        pub fn list_requirements(&self) -> Vec<String> {
            let mut requirements = Vec::new();
            if self.require_uppercase {
                requirements.push(String::from("Uppercase letter"));
            }
            if self.require_lowercase {
                requirements.push(String::from("Lowercase letter"));
            }
            if self.require_digit {
                requirements.push(String::from("Digit"));
            }
            if self.require_special_char {
                requirements.push(String::from("Special character"));
            }
            requirements
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_policy() {
        let mut policy = mdm_password_policy::PasswordPolicy::new(8, 16, true, true, true, true);
        assert!(!policy.is_valid("password"));
        assert!(policy.is_valid("Passw0rd!"));

        policy.set_min_length(5);
        assert!(policy.is_valid("Pass"));

        let requirements = policy.list_requirements();
        assert_eq!(
            requirements,
            vec![
                String::from("Uppercase letter"),
                String::from("Lowercase letter"),
                String::from("Digit"),
                String::from("Special character")
            ]
        ;
    }
}
