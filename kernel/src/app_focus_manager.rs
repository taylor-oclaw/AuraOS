extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AppFocusManager {
    entries: Vec<String>,
    active: bool,
}

impl AppFocusManager {
    pub fn new() -> Self { AppFocusManager { entries: Vec::new(), active: true } }
    pub fn add(&mut self, entry: &str) { self.entries.push(String::from(entry)); }
    pub fn count(&self) -> usize { self.entries.len() }
    pub fn is_active(&self) -> bool { self.active }
}
