extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct EmailIntelligence {
    emails: Vec<String>,
}

impl EmailIntelligence {
    pub fn new() -> Self {
        EmailIntelligence {
            emails: Vec::new(),
        }
    }

    pub fn add_email(&mut self, email: String) {
        self.emails.push(email);
    }

    pub fn get_emails(&self) -> &Vec<String> {
        &self.emails
    }

    pub fn search_emails(&self, keyword: &str) -> Vec<&String> {
        self.emails.iter().filter(|email| email.contains(keyword)).collect()
    }

    pub fn remove_email(&mut self, index: usize) -> Option<String> {
        if index < self.emails.len() {
            Some(self.emails.remove(index))
        } else {
            None
        }
    }

    pub fn count_emails(&self) -> usize {
        self.emails.len()
    }
}
