extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let email_forwarder = MiniAppEmailForward::new();
    email_forwarder.add_email("example@example.com");
    email_forwarder.forward_emails();
}

struct MiniAppEmailForward {
    emails: Vec<String>,
    forward_to: String,
}

impl MiniAppEmailForward {
    pub fn new() -> Self {
        MiniAppEmailForward {
            emails: Vec::new(),
            forward_to: String::from("default@example.com"),
        }
    }

    pub fn add_email(&mut self, email: &str) {
        self.emails.push(String::from(email));
    }

    pub fn set_forward_to(&mut self, address: &str) {
        self.forward_to = String::from(address);
    }

    pub fn get_emails(&self) -> &[String] {
        &self.emails
    }

    pub fn forward_emails(&self) {
        for email in &self.emails {
            // Simulate forwarding an email
        }
    }

    pub fn clear_emails(&mut self) {
        self.emails.clear();
    }
}
