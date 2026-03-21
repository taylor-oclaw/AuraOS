extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ParentalWebsiteBlock {
    blocked_websites: Vec<String>,
}

impl ParentalWebsiteBlock {
    pub fn new() -> Self {
        ParentalWebsiteBlock {
            blocked_websites: Vec::new(),
        }
    }

    pub fn add_website(&mut self, website: &str) {
        if !self.blocked_websites.contains(&website.to_string()) {
            self.blocked_websites.push(website.to_string());
        }
    }

    pub fn remove_website(&mut self, website: &str) {
        self.blocked_websites.retain(|w| w != website);
    }

    pub fn is_blocked(&self, website: &str) -> bool {
        self.blocked_websites.contains(&website.to_string())
    }

    pub fn list_blocked_websites(&self) -> Vec<String> {
        self.blocked_websites.clone()
    }

    pub fn clear_all(&mut self) {
        self.blocked_websites.clear();
    }
}
