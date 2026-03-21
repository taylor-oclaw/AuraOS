extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ActionItemDeadline {
    description: String,
    deadline: u64, // Unix timestamp in seconds
    completed: bool,
}

impl ActionItemDeadline {
    pub fn new(description: &str, deadline: u64) -> Self {
        ActionItemDeadline {
            description: String::from(description),
            deadline,
            completed: false,
        }
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn set_description(&mut self, new_description: &str) {
        self.description = String::from(new_description);
    }

    pub fn get_deadline(&self) -> u64 {
        self.deadline
    }

    pub fn set_deadline(&mut self, new_deadline: u64) {
        self.deadline = new_deadline;
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }

    pub fn mark_as_completed(&mut self) {
        self.completed = true;
    }
}
