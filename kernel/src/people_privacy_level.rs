extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

mod people_privacy_level {
    use super::*;

    pub struct PrivacyLevel {
        name: String,
        level: u8,
        data_access: Vec<String>,
    }

    impl PrivacyLevel {
        pub fn new(name: &str, level: u8) -> Self {
            PrivacyLevel {
                name: String::from(name),
                level,
                data_access: Vec::new(),
            }
        }

        pub fn get_name(&self) -> &str {
            &self.name
        }

        pub fn set_level(&mut self, level: u8) {
            self.level = level;
        }

        pub fn add_data_access(&mut self, access: &str) {
            if !self.data_access.contains(&String::from(access)) {
                self.data_access.push(String::from(access));
            }
        }

        pub fn remove_data_access(&mut self, access: &str) {
            self.data_access.retain(|a| a != access);
        }

        pub fn has_data_access(&self, access: &str) -> bool {
            self.data_access.contains(&String::from(access))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::people_privacy_level::*;

    #[test]
    fn test_privacy_level() {
        let mut privacy = PrivacyLevel::new("Alice", 3);
        assert_eq!(privacy.get_name(), "Alice");
        assert_eq!(privacy.level, 3);

        privacy.set_level(5);
        assert_eq!(privacy.level, 5);

        privacy.add_data_access("email");
        assert!(privacy.has_data_access("email"));

        privacy.remove_data_access("email");
        assert!(!privacy.has_data_access("email"));
    }
}
