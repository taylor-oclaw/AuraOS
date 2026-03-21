extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AutoTaskEmailSend {
    server: String,
    port: u16,
    username: String,
    password: String,
    recipient: String,
}

impl AutoTaskEmailSend {
    pub fn new(server: &str, port: u16, username: &str, password: &str, recipient: &str) -> Self {
        AutoTaskEmailSend {
            server: String::from(server),
            port,
            username: String::from(username),
            password: String::from(password),
            recipient: String::from(recipient),
        }
    }

    pub fn set_server(&mut self, server: &str) {
        self.server = String::from(server);
    }

    pub fn set_port(&mut self, port: u16) {
        self.port = port;
    }

    pub fn set_username(&mut self, username: &str) {
        self.username = String::from(username);
    }

    pub fn set_password(&mut self, password: &str) {
        self.password = String::from(password);
    }

    pub fn set_recipient(&mut self, recipient: &str) {
        self.recipient = String::from(recipient);
    }

    pub fn send_email(&self, subject: &str, body: &str) -> Result<(), &'static str> {
        // Simulate email sending logic
        if self.server.is_empty() || self.username.is_empty() || self.password.is_empty() || self.recipient.is_empty() {
            return Err("Missing required fields");
        }

        // Placeholder for actual email sending logic
        println!("Sending email to {} with subject: {}", self.recipient, subject);
        Ok(())
    }
}
