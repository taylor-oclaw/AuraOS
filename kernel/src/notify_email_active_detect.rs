extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct NotifyEmailActiveDetect {
    active_emails: Vec<String>,
}

impl NotifyEmailActiveDetect {
    pub fn new() -> Self {
        NotifyEmailActiveDetect {
            active_emails: Vec::new(),
        }
    }

    pub fn add_email(&mut self, email: String) {
        if !self.active_emails.contains(&email) {
            self.active_emails.push(email);
        }
    }

    pub fn remove_email(&mut self, email: &str) -> bool {
        let pos = self.active_emails.iter().position(|e| e == email);
        if let Some(index) = pos {
            self.active_emails.remove(index);
            true
        } else {
            false
        }
    }

    pub fn is_email_active(&self, email: &str) -> bool {
        self.active_emails.contains(&String::from(email))
    }

    pub fn get_all_active_emails(&self) -> Vec<String> {
        self.active_emails.clone()
    }

    pub fn clear_all_emails(&mut self) {
        self.active_emails.clear();
    }
}
