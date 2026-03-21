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
    pub fn count(&self) -> usize { self.entries.len() }
    pub fn is_active_state(&self) -> bool { self.active }
}

pub fn init(_width: u32, _height: u32) {}
pub fn render(_fb: &mut [u8], _width: usize, _bpp: usize) {}
pub fn handle_key(_scancode: u8) {}
pub fn handle_mouse(_x: i32, _y: i32, _buttons: u8) {}
pub fn update_mouse(_x: i32, _y: i32, _buttons: u8) {}
pub fn is_active() -> bool { false }
