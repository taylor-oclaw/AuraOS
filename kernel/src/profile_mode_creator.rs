extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct ProfileModeCreator {
    modes: Vec<String>,
    current_mode: usize,
}

impl ProfileModeCreator {
    pub fn new() -> Self {
        ProfileModeCreator {
            modes: Vec::new(),
            current_mode: 0,
        }
    }

    pub fn add_mode(&mut self, mode_name: &str) {
        self.modes.push(String::from(mode_name));
    }

    pub fn remove_mode(&mut self, mode_name: &str) -> bool {
        if let Some(index) = self.modes.iter().position(|m| m == mode_name) {
            self.modes.remove(index);
            true
        } else {
            false
        }
    }

    pub fn switch_to_next_mode(&mut self) {
        if !self.modes.is_empty() {
            self.current_mode = (self.current_mode + 1) % self.modes.len();
        }
    }

    pub fn get_current_mode(&self) -> Option<&str> {
        self.modes.get(self.current_mode).map(|s| s.as_str())
    }

    pub fn list_modes(&self) -> Vec<&str> {
        self.modes.iter().map(|s| s.as_str()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_mode_creator() {
        let mut creator = ProfileModeCreator::new();
        assert_eq!(creator.list_modes(), vec![]);

        creator.add_mode("mode1");
        creator.add_mode("mode2");
        assert_eq!(creator.list_modes(), vec!["mode1", "mode2"]);

        assert!(creator.remove_mode("mode1"));
        assert_eq!(creator.list_modes(), vec!["mode2"]);

        assert!(!creator.remove_mode("mode3"));
        assert_eq!(creator.list_modes(), vec!["mode2"]);

        creator.switch_to_next_mode();
        assert_eq!(creator.get_current_mode(), Some("mode2"));

        creator.add_mode("mode1");
        creator.switch_to_next_mode();
        assert_eq!(creator.get_current_mode(), Some("mode1"));
    }
}
