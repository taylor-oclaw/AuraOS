extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AutoTaskEmailDraft {
    subject: String,
    body: String,
    recipients: Vec<String>,
    cc: Vec<String>,
    bcc: Vec<String>,
}

impl AutoTaskEmailDraft {
    pub fn new(subject: &str, body: &str) -> Self {
        AutoTaskEmailDraft {
            subject: String::from(subject),
            body: String::from(body),
            recipients: Vec::new(),
            cc: Vec::new(),
            bcc: Vec::new(),
        }
    }

    pub fn add_recipient(&mut self, recipient: &str) {
        self.recipients.push(String::from(recipient));
    }

    pub fn add_cc(&mut self, cc: &str) {
        self.cc.push(String::from(cc));
    }

    pub fn add_bcc(&mut self, bcc: &str) {
        self.bcc.push(String::from(bcc));
    }

    pub fn get_subject(&self) -> &str {
        &self.subject
    }

    pub fn get_body(&self) -> &str {
        &self.body
    }

    pub fn get_recipients(&self) -> &[String] {
        &self.recipients
    }

    pub fn get_cc(&self) -> &[String] {
        &self.cc
    }

    pub fn get_bcc(&self) -> &[String] {
        &self.bcc
    }
}
