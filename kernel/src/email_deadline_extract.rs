extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct EmailDeadlineExtractor {
    emails: Vec<String>,
}

impl EmailDeadlineExtractor {
    pub fn new() -> Self {
        EmailDeadlineExtractor {
            emails: Vec::new(),
        }
    }

    pub fn add_email(&mut self, email: String) {
        self.emails.push(email);
    }

    pub fn get_emails(&self) -> &Vec<String> {
        &self.emails
    }

    pub fn extract_deadlines(&self) -> Vec<String> {
        let mut deadlines = Vec::new();
        for email in &self.emails {
            if let Some(deadline) = Self::find_deadline(email) {
                deadlines.push(deadline);
            }
        }
        deadlines
    }

    fn find_deadline(email: &str) -> Option<String> {
        // Simple heuristic to find a deadline in the format "Due: YYYY-MM-DD"
        for line in email.lines() {
            if let Some(pos) = line.find("Due:") {
                let rest_of_line = &line[pos + 4..];
                if let Some(end_pos) = rest_of_line.find(|c: char| !c.is_digit(10) && c != '-') {
                    return Some(rest_of_line[..end_pos].to_string());
                }
            }
        }
        None
    }

    pub fn clear_emails(&mut self) {
        self.emails.clear();
    }
}
