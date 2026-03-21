extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn meeting_confidential_flag_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn meeting_confidential_flag_exit() {
    // Cleanup logic for the module
}

pub struct MeetingConfidentialFlag {
    flags: Vec<String>,
}

impl MeetingConfidentialFlag {
    pub fn new() -> Self {
        MeetingConfidentialFlag { flags: Vec::new() }
    }

    pub fn add_flag(&mut self, flag: &str) {
        self.flags.push(String::from(flag));
    }

    pub fn remove_flag(&mut self, flag: &str) {
        if let Some(index) = self.flags.iter().position(|f| f == flag) {
            self.flags.remove(index);
        }
    }

    pub fn has_flag(&self, flag: &str) -> bool {
        self.flags.contains(&String::from(flag))
    }

    pub fn list_flags(&self) -> Vec<String> {
        self.flags.clone()
    }

    pub fn clear_flags(&mut self) {
        self.flags.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meeting_confidential_flag() {
        let mut flag_manager = MeetingConfidentialFlag::new();

        assert_eq!(flag_manager.list_flags().len(), 0);
        assert!(!flag_manager.has_flag("FLAG1"));

        flag_manager.add_flag("FLAG1");
        assert_eq!(flag_manager.list_flags().len(), 1);
        assert!(flag_manager.has_flag("FLAG1"));

        flag_manager.remove_flag("FLAG1");
        assert_eq!(flag_manager.list_flags().len(), 0);
        assert!(!flag_manager.has_flag("FLAG1"));

        flag_manager.add_flag("FLAG2");
        flag_manager.clear_flags();
        assert_eq!(flag_manager.list_flags().len(), 0);
        assert!(!flag_manager.has_flag("FLAG2"));
    }
}
