extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct Desktop {
    entries: Vec<String>,
    active: bool,
}

impl Desktop {
    pub fn new() -> Self {
        Desktop { entries: Vec::new(), active: true }
    }
    pub fn add(&mut self, entry: &str) { self.entries.push(String::from(entry)); }
    pub fn remove(&mut self, entry: &str) { self.entries.retain(|e| e != entry); }
    pub fn count(&self) -> usize { self.entries.len() }
    pub fn is_active(&self) -> bool { self.active }
}

pub fn init(_w: u32, _h: u32) {}
pub fn render() {}
pub fn handle_key(_sc: u8) {}
pub fn handle_mouse(_x: i32, _y: i32, _b: u8) {}
pub fn update_mouse(_x: i32, _y: i32, _b: u8) {}
pub fn is_active() -> bool { false }
