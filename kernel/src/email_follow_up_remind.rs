extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn email_follow_up_remind_init() {
    // Initialization logic for the module
}

pub extern "C" fn email_follow_up_remind_exit() {
    // Cleanup logic for the module
}

pub struct EmailFollowUpRemind {
    emails: Vec<String>,
    reminders: Vec<String>,
}

impl EmailFollowUpRemind {
    pub fn new() -> Self {
        EmailFollowUpRemind {
            emails: Vec::new(),
            reminders: Vec::new(),
        }
    }

    pub fn add_email(&mut self, email: String) {
        self.emails.push(email);
    }

    pub fn remove_email(&mut self, index: usize) -> Option<String> {
        if index < self.emails.len() {
            Some(self.emails.remove(index))
        } else {
            None
        }
    }

    pub fn add_reminder(&mut self, reminder: String) {
        self.reminders.push(reminder);
    }

    pub fn remove_reminder(&mut self, index: usize) -> Option<String> {
        if index < self.reminders.len() {
            Some(self.reminders.remove(index))
        } else {
            None
        }
    }

    pub fn list_emails(&self) -> &[String] {
        &self.emails
    }

    pub fn list_reminders(&self) -> &[String] {
        &self.reminders
    }
}
