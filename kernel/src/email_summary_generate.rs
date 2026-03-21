extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct EmailSummaryGenerator {
    emails: Vec<String>,
}

impl EmailSummaryGenerator {
    pub fn new() -> Self {
        EmailSummaryGenerator {
            emails: Vec::new(),
        }
    }

    pub fn add_email(&mut self, email: String) {
        self.emails.push(email);
    }

    pub fn get_emails_count(&self) -> usize {
        self.emails.len()
    }

    pub fn get_summary(&self) -> String {
        let mut summary = String::from("Email Summary:\n");
        for (index, email) in self.emails.iter().enumerate() {
            summary.push_str(&format!("Email {}:\n{}\n\n", index + 1, email));
        }
        summary
    }

    pub fn clear_emails(&mut self) {
        self.emails.clear();
    }

    pub fn get_email_content(&self, index: usize) -> Option<String> {
        if index < self.emails.len() {
            Some(self.emails[index].clone())
        } else {
            None
        }
    }
}
