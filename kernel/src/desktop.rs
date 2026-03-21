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

pub fn init(_width: u32, _height: u32) {
    // Desktop initialization placeholder
}

pub fn render() {
    // Desktop render placeholder
}

pub fn handle_mouse(_x: i32, _y: i32, _buttons: u8) {
    // Mouse handler placeholder
}

pub fn handle_key(_scancode: u8) {
    // Key handler placeholder
}

pub fn update_mouse(_x: i32, _y: i32, _buttons: u8) {}
pub fn is_active() -> bool { false }
