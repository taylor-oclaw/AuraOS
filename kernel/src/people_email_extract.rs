extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct PeopleEmailExtractor {
    emails: Vec<String>,
}

impl PeopleEmailExtractor {
    pub fn new() -> Self {
        PeopleEmailExtractor {
            emails: Vec::new(),
        }
    }

    pub fn add_email(&mut self, email: String) {
        if Self::is_valid_email(&email) {
            self.emails.push(email);
        }
    }

    pub fn remove_email(&mut self, email: &str) -> bool {
        let pos = self.emails.iter().position(|e| e == email);
        if let Some(index) = pos {
            self.emails.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_emails(&self) -> Vec<String> {
        self.emails.clone()
    }

    pub fn count_emails(&self) -> usize {
        self.emails.len()
    }

    pub fn contains_email(&self, email: &str) -> bool {
        self.emails.contains(&String::from(email))
    }

    fn is_valid_email(email: &str) -> bool {
        let parts: Vec<&str> = email.split('@').collect();
        parts.len() == 2 && !parts[0].is_empty() && !parts[1].is_empty()
    }
}
