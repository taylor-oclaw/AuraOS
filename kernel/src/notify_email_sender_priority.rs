extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct NotifyEmailSenderPriority {
    emails: Vec<String>,
    priority_levels: Vec<u8>,
}

impl NotifyEmailSenderPriority {
    pub fn new() -> Self {
        NotifyEmailSenderPriority {
            emails: Vec::new(),
            priority_levels: Vec::new(),
        }
    }

    pub fn add_email(&mut self, email: String, priority: u8) {
        self.emails.push(email);
        self.priority_levels.push(priority);
    }

    pub fn get_emails_by_priority(&self, priority: u8) -> Vec<String> {
        self.emails
            .iter()
            .zip(self.priority_levels.iter())
            .filter(|&(_, &p)| p == priority)
            .map(|(email, _)| email.clone())
            .collect()
    }

    pub fn remove_email(&mut self, email: &str) -> bool {
        if let Some(index) = self.emails.iter().position(|e| e == email) {
            self.emails.remove(index);
            self.priority_levels.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_highest_priority_email(&self) -> Option<String> {
        self.emails
            .iter()
            .zip(self.priority_levels.iter())
            .max_by_key(|&(_, &priority)| priority)
            .map(|(email, _)| email.clone())
    }

    pub fn list_emails(&self) -> Vec<(String, u8)> {
        self.emails
            .iter()
            .zip(self.priority_levels.iter())
            .map(|(email, &priority)| (email.clone(), priority))
            .collect()
    }
}
