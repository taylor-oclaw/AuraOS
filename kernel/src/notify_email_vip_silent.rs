extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct NotifyEmailVipSilent {
    email_list: Vec<String>,
    subject: String,
    body: String,
    is_silent: bool,
}

impl NotifyEmailVipSilent {
    pub fn new(subject: &str, body: &str) -> Self {
        NotifyEmailVipSilent {
            email_list: Vec::new(),
            subject: String::from(subject),
            body: String::from(body),
            is_silent: false,
        }
    }

    pub fn add_email(&mut self, email: &str) {
        self.email_list.push(String::from(email));
    }

    pub fn remove_email(&mut self, email: &str) {
        if let Some(pos) = self.email_list.iter().position(|e| e == email) {
            self.email_list.remove(pos);
        }
    }

    pub fn set_subject(&mut self, subject: &str) {
        self.subject = String::from(subject);
    }

    pub fn set_body(&mut self, body: &str) {
        self.body = String::from(body);
    }

    pub fn toggle_silent_mode(&mut self) {
        self.is_silent = !self.is_silent;
    }
}
