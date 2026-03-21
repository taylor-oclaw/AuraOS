extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

struct BizClientProfile {
    name: String,
    age: u32,
    email: String,
    interests: Vec<String>,
    active: bool,
}

impl BizClientProfile {
    pub fn new(name: &str, age: u32, email: &str) -> Self {
        BizClientProfile {
            name: String::from(name),
            age,
            email: String::from(email),
            interests: Vec::new(),
            active: true,
        }
    }

    pub fn add_interest(&mut self, interest: &str) {
        self.interests.push(String::from(interest));
    }

    pub fn remove_interest(&mut self, interest: &str) -> bool {
        if let Some(pos) = self.interests.iter().position(|i| i == interest) {
            self.interests.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_biz_client_profile() {
        let mut profile = BizClientProfile::new("Alice", 30, "alice@example.com");
        assert_eq!(profile.get_name(), "Alice");
        assert!(profile.is_active());

        profile.add_interest("AI");
        profile.add_interest("Rust");
        assert_eq!(profile.interests.len(), 2);

        assert!(profile.remove_interest("AI"));
        assert!(!profile.remove_interest("AI"));
        assert_eq!(profile.interests.len(), 1);

        profile.set_active(false);
        assert!(!profile.is_active());
    }
}
