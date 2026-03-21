extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AuraAssistantRemind {
    reminders: Vec<String>,
}

impl AuraAssistantRemind {
    pub fn new() -> Self {
        AuraAssistantRemind {
            reminders: Vec::new(),
        }
    }

    pub fn add_reminder(&mut self, reminder: &str) {
        self.reminders.push(String::from(reminder));
    }

    pub fn remove_reminder(&mut self, index: usize) -> Option<String> {
        if index < self.reminders.len() {
            Some(self.reminders.remove(index))
        } else {
            None
        }
    }

    pub fn get_reminder(&self, index: usize) -> Option<&String> {
        self.reminders.get(index)
    }

    pub fn list_reminders(&self) -> &[String] {
        &self.reminders
    }

    pub fn clear_reminders(&mut self) {
        self.reminders.clear();
    }
}
