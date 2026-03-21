extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

#[derive(Debug)]
pub struct PrefDetect {
    pub preferences: Vec<String>,
}

impl PrefDetect {
    pub fn new() -> Self {
        PrefDetect {
            preferences: Vec::new(),
        }
    }

    pub fn add_preference(&mut self, preference: String) {
        self.preferences.push(preference);
    }

    pub fn remove_preference(&mut self, preference: &str) -> bool {
        if let Some(index) = self.preferences.iter().position(|p| p == preference) {
            self.preferences.remove(index);
            true
        } else {
            false
        }
    }

    pub fn has_preference(&self, preference: &str) -> bool {
        self.preferences.contains(&String::from(preference))
    }

    pub fn list_preferences(&self) -> Vec<String> {
        self.preferences.clone()
    }

    pub fn clear_preferences(&mut self) {
        self.preferences.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pref_detect() {
        let mut pd = PrefDetect::new();
        assert_eq!(pd.list_preferences().len(), 0);

        pd.add_preference(String::from("AI"));
        pd.add_preference(String::from("Machine Learning"));

        assert_eq!(pd.list_preferences().len(), 2);
        assert!(pd.has_preference("AI"));
        assert!(!pd.has_preference("Blockchain"));

        assert!(pd.remove_preference("AI"));
        assert!(!pd.has_preference("AI"));
        assert_eq!(pd.list_preferences().len(), 1);

        pd.clear_preferences();
        assert_eq!(pd.list_preferences().len(), 0);
    }
}
