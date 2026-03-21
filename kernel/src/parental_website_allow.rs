extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct ParentalWebsiteAllow {
    allowed_websites: Vec<String>,
}

impl ParentalWebsiteAllow {
    pub fn new() -> Self {
        ParentalWebsiteAllow {
            allowed_websites: Vec::new(),
        }
    }

    pub fn add_website(&mut self, website: &str) {
        if !self.allowed_websites.contains(&String::from(website)) {
            self.allowed_websites.push(String::from(website));
        }
    }

    pub fn remove_website(&mut self, website: &str) {
        self.allowed_websites.retain(|w| w != website);
    }

    pub fn is_allowed(&self, website: &str) -> bool {
        self.allowed_websites.contains(&String::from(website))
    }

    pub fn list_allowed_websites(&self) -> Vec<String> {
        self.allowed_websites.clone()
    }

    pub fn clear_all(&mut self) {
        self.allowed_websites.clear();
    }
}
