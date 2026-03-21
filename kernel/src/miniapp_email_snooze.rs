extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut email_snooze = MiniAppEmailSnooze::new();
    email_snooze.add_email("example@example.com", 60);
    email_snooze.snooze_all_emails(120);
    email_snooze.remove_email("example@example.com");
    email_snooze.list_emails();
}

pub struct Email {
    address: String,
    snooze_time: u32, // in minutes
}

impl Email {
    fn new(address: &str, snooze_time: u32) -> Self {
        Email {
            address: String::from(address),
            snooze_time,
        }
    }
}

pub struct MiniAppEmailSnooze {
    emails: Vec<Email>,
}

impl MiniAppEmailSnooze {
    pub fn new() -> Self {
        MiniAppEmailSnooze { emails: Vec::new() }
    }

    pub fn add_email(&mut self, address: &str, snooze_time: u32) {
        let email = Email::new(address, snooze_time);
        self.emails.push(email);
    }

    pub fn remove_email(&mut self, address: &str) {
        self.emails.retain(|e| e.address != address);
    }

    pub fn snooze_all_emails(&mut self, additional_time: u32) {
        for email in &mut self.emails {
            email.snooze_time += additional_time;
        }
    }

    pub fn list_emails(&self) {
        for email in &self.emails {
            // Simulate printing
        }
    }

    pub fn get_email_snooze_time(&self, address: &str) -> Option<u32> {
        self.emails.iter().find(|e| e.address == address).map(|e| e.snooze_time)
    }
}
