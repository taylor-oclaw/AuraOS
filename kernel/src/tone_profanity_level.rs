extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

mod tone_profanity_level {
    use super::*;

    pub struct ProfanityLevel {
        levels: Vec<String>,
    }

    impl ProfanityLevel {
        pub fn new() -> Self {
            let mut levels = Vec::new();
            levels.push(String::from("Mild"));
            levels.push(String::from("Moderate"));
            levels.push(String::from("Severe"));
            levels.push(String::from("Extreme"));
            levels.push(String::from("Offensive"));

            ProfanityLevel { levels }
        }

        pub fn get_level(&self, index: usize) -> Option<&str> {
            self.levels.get(index).map(|s| s.as_str())
        }

        pub fn add_level(&mut self, level: String) {
            self.levels.push(level);
        }

        pub fn remove_level(&mut self, index: usize) -> Option<String> {
            if index < self.levels.len() {
                Some(self.levels.remove(index))
            } else {
                None
            }
        }

        pub fn update_level(&mut self, index: usize, new_level: String) -> bool {
            if index < self.levels.len() {
                self.levels[index] = new_level;
                true
            } else {
                false
            }
        }

        pub fn list_levels(&self) -> Vec<&str> {
            self.levels.iter().map(|s| s.as_str()).collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::tone_profanity_level::*;

    #[test]
    fn test_new() {
        let profanity = ProfanityLevel::new();
        assert_eq!(profanity.levels.len(), 5);
    }

    #[test]
    fn test_get_level() {
        let profanity = ProfanityLevel::new();
        assert_eq!(profanity.get_level(0), Some("Mild"));
        assert_eq!(profanity.get_level(10), None);
    }

    #[test]
    fn test_add_level() {
        let mut profanity = ProfanityLevel::new();
        profanity.add_level(String::from("Custom"));
        assert_eq!(profanity.levels.len(), 6);
        assert_eq!(profanity.get_level(5), Some("Custom"));
    }

    #[test]
    fn test_remove_level() {
        let mut profanity = ProfanityLevel::new();
        assert_eq!(profanity.remove_level(0), Some(String::from("Mild")));
        assert_eq!(profanity.levels.len(), 4);
        assert_eq!(profanity.get_level(0), Some("Moderate"));
    }

    #[test]
    fn test_update_level() {
        let mut profanity = ProfanityLevel::new();
        assert!(profanity.update_level(2, String::from("Updated")));
        assert_eq!(profanity.get_level(2), Some("Updated"));
        assert!(!profanity.update_level(10, String::from("Invalid")));
    }

    #[test]
    fn test_list_levels() {
        let profanity = ProfanityLevel::new();
        let levels = profanity.list_levels();
        assert_eq!(levels.len(), 5);
        assert_eq!(levels[0], "Mild");
        assert_eq!(levels[1], "Moderate");
        assert_eq!(levels[2], "Severe");
        assert_eq!(levels[3], "Extreme");
        assert_eq!(levels[4], "Offensive");
    }
}
