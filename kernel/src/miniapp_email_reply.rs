extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MiniAppEmailReply {
    subject: String,
    body: String,
    recipient: String,
    sender: String,
    cc: Vec<String>,
}

impl MiniAppEmailReply {
    pub fn new(subject: &str, body: &str, recipient: &str, sender: &str) -> Self {
        MiniAppEmailReply {
            subject: String::from(subject),
            body: String::from(body),
            recipient: String::from(recipient),
            sender: String::from(sender),
            cc: Vec::new(),
        }
    }

    pub fn add_cc(&mut self, email: &str) {
        self.cc.push(String::from(email));
    }

    pub fn get_subject(&self) -> &str {
        &self.subject
    }

    pub fn set_subject(&mut self, subject: &str) {
        self.subject = String::from(subject);
    }

    pub fn get_body(&self) -> &str {
        &self.body
    }

    pub fn set_body(&mut self, body: &str) {
        self.body = String::from(body);
    }

    pub fn get_recipient(&self) -> &str {
        &self.recipient
    }

    pub fn set_recipient(&mut self, recipient: &str) {
        self.recipient = String::from(recipient);
    }

    pub fn get_sender(&self) -> &str {
        &self.sender
    }

    pub fn set_sender(&mut self, sender: &str) {
        self.sender = String::from(sender);
    }

    pub fn get_cc(&self) -> &[String] {
        &self.cc
    }
}
