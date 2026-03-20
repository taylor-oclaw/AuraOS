extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

struct AgentPreferenceLearn {
    preferences: Vec<String>,
}

impl AgentPreferenceLearn {
    pub fn new() -> Self {
        AgentPreferenceLearn {
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

    pub fn get_preferences(&self) -> &[String] {
        &self.preferences
    }

    pub fn has_preference(&self, preference: &str) -> bool {
        self.preferences.contains(&preference.to_string())
    }

    pub fn clear_preferences(&mut self) {
        self.preferences.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_preference_learn() {
        let mut agent = AgentPreferenceLearn::new();

        assert_eq!(agent.get_preferences().len(), 0);

        agent.add_preference(String::from("AI"));
        agent.add_preference(String::from("Machine Learning"));

        assert_eq!(agent.get_preferences().len(), 2);
        assert!(agent.has_preference("AI"));
        assert!(!agent.has_preference("Deep Learning"));

        assert!(agent.remove_preference("AI"));
        assert!(!agent.remove_preference("AI"));

        assert_eq!(agent.get_preferences().len(), 1);

        agent.clear_preferences();
        assert_eq!(agent.get_preferences().len(), 0);
    }
}
