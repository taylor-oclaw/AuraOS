extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

#[derive(Debug)]
pub struct EmailDraft {
    subject: String,
    body: String,
    to: Vec<String>,
    cc: Vec<String>,
    bcc: Vec<String>,
}

impl EmailDraft {
    pub fn new(subject: &str, body: &str) -> Self {
        EmailDraft {
            subject: String::from(subject),
            body: String::from(body),
            to: Vec::new(),
            cc: Vec::new(),
            bcc: Vec::new(),
        }
    }

    pub fn add_to(&mut self, email: &str) {
        self.to.push(String::from(email));
    }

    pub fn add_cc(&mut self, email: &str) {
        self.cc.push(String::from(email));
    }

    pub fn add_bcc(&mut self, email: &str) {
        self.bcc.push(String::from(email));
    }

    pub fn get_subject(&self) -> &str {
        &self.subject
    }

    pub fn get_body(&self) -> &str {
        &self.body
    }

    pub fn get_recipients(&self) -> (Vec<&str>, Vec<&str>, Vec<&str>) {
        (
            self.to.iter().map(|s| s.as_str()).collect(),
            self.cc.iter().map(|s| s.as_str()).collect(),
            self.bcc.iter().map(|s| s.as_str()).collect(),
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_draft() {
        let mut draft = EmailDraft::new("Test Subject", "Hello, this is a test email.");
        draft.add_to("to@example.com");
        draft.add_cc("cc@example.com");
        draft.add_bcc("bcc@example.com");

        assert_eq!(draft.get_subject(), "Test Subject");
        assert_eq!(draft.get_body(), "Hello, this is a test email.");

        let (to, cc, bcc) = draft.get_recipients();
        assert_eq!(to, vec!["to@example.com"]);
        assert_eq!(cc, vec!["cc@example.com"]);
        assert_eq!(bcc, vec!["bcc@example.com"]);
    }
}
