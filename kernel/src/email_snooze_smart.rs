extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct EmailSnoozeSmart {
    emails: Vec<String>,
    snoozed_until: Vec<u64>, // Unix timestamps
}

impl EmailSnoozeSmart {
    pub fn new() -> Self {
        EmailSnoozeSmart {
            emails: Vec::new(),
            snoozed_until: Vec::new(),
        }
    }

    pub fn add_email(&mut self, email: String) {
        self.emails.push(email);
        self.snoozed_until.push(0); // Initially not snoozed
    }

    pub fn snooze_email(&mut self, index: usize, until: u64) -> bool {
        if index < self.emails.len() {
            self.snoozed_until[index] = until;
            true
        } else {
            false
        }
    }

    pub fn is_snoozed(&self, index: usize) -> bool {
        if index < self.emails.len() {
            let current_time = 1633072800; // Example timestamp, replace with actual time source
            self.snoozed_until[index] > current_time
        } else {
            false
        }
    }

    pub fn get_email(&self, index: usize) -> Option<&String> {
        if index < self.emails.len() {
            Some(&self.emails[index])
        } else {
            None
        }
    }

    pub fn remove_snooze(&mut self, index: usize) -> bool {
        if index < self.emails.len() {
            self.snoozed_until[index] = 0;
            true
        } else {
            false
        }
    }
}
