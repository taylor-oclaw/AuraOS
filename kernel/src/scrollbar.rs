extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct Scrollbar {
    entries: Vec<String>,
    active: bool,
}

impl Scrollbar {
    pub fn new() -> Self {
        Scrollbar { entries: Vec::new(), active: true }
    }
    pub fn add(&mut self, entry: &str) { self.entries.push(String::from(entry)); }
    pub fn remove(&mut self, entry: &str) { self.entries.retain(|e| e != entry); }
    pub fn count(&self) -> usize { self.entries.len() }
    pub fn is_active(&self) -> bool { self.active }
}
