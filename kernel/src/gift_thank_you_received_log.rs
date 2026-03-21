extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct GiftThankYouReceivedLog {
    entries: Vec<String>,
}

impl GiftThankYouReceivedLog {
    pub fn new() -> Self {
        GiftThankYouReceivedLog {
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, entry: String) {
        self.entries.push(entry);
    }

    pub fn get_entries(&self) -> &Vec<String> {
        &self.entries
    }

    pub fn clear_entries(&mut self) {
        self.entries.clear();
    }

    pub fn count_entries(&self) -> usize {
        self.entries.len()
    }

    pub fn find_entry(&self, keyword: &str) -> Option<&String> {
        self.entries.iter().find(|entry| entry.contains(keyword))
    }
}
