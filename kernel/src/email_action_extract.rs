extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct EmailActionExtract {
    actions: Vec<String>,
}

impl EmailActionExtract {
    pub fn new() -> Self {
        EmailActionExtract {
            actions: Vec::new(),
        }
    }

    pub fn add_action(&mut self, action: String) {
        self.actions.push(action);
    }

    pub fn remove_action(&mut self, index: usize) -> Option<String> {
        if index < self.actions.len() {
            Some(self.actions.remove(index))
        } else {
            None
        }
    }

    pub fn get_actions(&self) -> &Vec<String> {
        &self.actions
    }

    pub fn clear_actions(&mut self) {
        self.actions.clear();
    }

    pub fn contains_action(&self, action: &str) -> bool {
        self.actions.iter().any(|a| a == action)
    }
}
