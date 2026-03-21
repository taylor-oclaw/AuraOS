extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MeetingActionExtractor {
    actions: Vec<String>,
}

impl MeetingActionExtractor {
    pub fn new() -> Self {
        MeetingActionExtractor {
            actions: Vec::new(),
        }
    }

    pub fn add_action(&mut self, action: String) {
        self.actions.push(action);
    }

    pub fn get_actions(&self) -> &Vec<String> {
        &self.actions
    }

    pub fn clear_actions(&mut self) {
        self.actions.clear();
    }

    pub fn has_action(&self, action: &str) -> bool {
        self.actions.iter().any(|a| a == action)
    }

    pub fn count_actions(&self) -> usize {
        self.actions.len()
    }
}
