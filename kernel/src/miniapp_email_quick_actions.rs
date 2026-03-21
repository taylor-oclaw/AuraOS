extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    println!("MiniApp Email Quick Actions module loaded!");
    0
}

pub struct MiniAppEmailQuickActions {
    emails: Vec<String>,
}

impl MiniAppEmailQuickActions {
    pub fn new() -> Self {
        MiniAppEmailQuickActions {
            emails: Vec::new(),
        }
    }

    pub fn add_email(&mut self, email: String) {
        self.emails.push(email);
    }

    pub fn remove_email(&mut self, index: usize) -> Option<String> {
        if index < self.emails.len() {
            Some(self.emails.remove(index))
        } else {
            None
        }
    }

    pub fn get_emails(&self) -> &[String] {
        &self.emails
    }

    pub fn mark_as_read(&mut self, index: usize) {
        if let Some(email) = self.emails.get_mut(index) {
            email.push_str(" [READ]");
        }
    }

    pub fn search_emails(&self, query: &str) -> Vec<String> {
        self.emails
            .iter()
            .filter(|email| email.contains(query))
            .cloned()
            .collect()
    }
}
