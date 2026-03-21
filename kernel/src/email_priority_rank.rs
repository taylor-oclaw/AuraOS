extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct EmailPriorityRank {
    emails: Vec<String>,
    priorities: Vec<u8>,
}

impl EmailPriorityRank {
    pub fn new() -> Self {
        EmailPriorityRank {
            emails: Vec::new(),
            priorities: Vec::new(),
        }
    }

    pub fn add_email(&mut self, email: String, priority: u8) {
        self.emails.push(email);
        self.priorities.push(priority);
    }

    pub fn get_emails(&self) -> &Vec<String> {
        &self.emails
    }

    pub fn get_priorities(&self) -> &Vec<u8> {
        &self.priorities
    }

    pub fn get_highest_priority_email(&self) -> Option<&String> {
        self.emails.iter().zip(self.priorities.iter())
            .max_by_key(|&(_, &priority)| priority)
            .map(|(email, _)| email)
    }

    pub fn remove_email(&mut self, index: usize) {
        if index < self.emails.len() {
            self.emails.remove(index);
            self.priorities.remove(index);
        }
    }
}
