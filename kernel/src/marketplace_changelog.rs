extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct MarketplaceChangelog {
    entries: Vec<String>,
}

impl MarketplaceChangelog {
    pub fn new() -> Self {
        MarketplaceChangelog {
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, entry: String) {
        self.entries.push(entry);
    }

    pub fn get_entries(&self) -> &Vec<String> {
        &self.entries
    }

    pub fn remove_entry(&mut self, index: usize) -> Option<String> {
        if index < self.entries.len() {
            Some(self.entries.remove(index))
        } else {
            None
        }
    }

    pub fn clear_entries(&mut self) {
        self.entries.clear();
    }

    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }
}
