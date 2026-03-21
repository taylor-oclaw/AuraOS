extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct EmailThread {
    subject: String,
    emails: Vec<String>,
}

impl EmailThread {
    pub fn new(subject: &str) -> Self {
        EmailThread {
            subject: String::from(subject),
            emails: Vec::new(),
        }
    }

    pub fn add_email(&mut self, email_content: &str) {
        self.emails.push(String::from(email_content));
    }

    pub fn get_subject(&self) -> &str {
        &self.subject
    }

    pub fn get_emails_count(&self) -> usize {
        self.emails.len()
    }

    pub fn get_email_content(&self, index: usize) -> Option<&str> {
        if index < self.emails.len() {
            Some(&self.emails[index])
        } else {
            None
        }
    }

    pub fn summarize_thread(&self) -> String {
        let mut summary = String::from("info");
        for (i, email) in self.emails.iter().enumerate() {
            summary.push_str(&String::from("info"));
        }
        summary
    }
}
