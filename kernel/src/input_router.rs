extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct InputRouter {
    entries: Vec<String>,
    active: bool,
}

impl InputRouter {
    pub fn new() -> Self {
        InputRouter { entries: Vec::new(), active: true }
    }
    pub fn count(&self) -> usize { self.entries.len() }
}

pub fn handle_key(_character: char) {}
