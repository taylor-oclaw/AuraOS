extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn tone_vendor_mode_init() -> i32 {
    0
}

pub extern "C" fn tone_vendor_mode_exit() -> i32 {
    0
}

pub struct ToneVendorMode {
    modes: Vec<String>,
    current_mode_index: usize,
}

impl ToneVendorMode {
    pub fn new(modes: Vec<&str>) -> Self {
        let mode_strings: Vec<String> = modes.into_iter().map(String::from).collect();
        ToneVendorMode {
            modes: mode_strings,
            current_mode_index: 0,
        }
    }

    pub fn add_mode(&mut self, mode: &str) {
        self.modes.push(String::from(mode));
    }

    pub fn remove_mode(&mut self, index: usize) -> Option<String> {
        if index < self.modes.len() {
            Some(self.modes.remove(index))
        } else {
            None
        }
    }

    pub fn get_current_mode(&self) -> &str {
        &self.modes[self.current_mode_index]
    }

    pub fn set_next_mode(&mut self) {
        if self.current_mode_index < self.modes.len() - 1 {
            self.current_mode_index += 1;
        } else {
            self.current_mode_index = 0;
        }
    }

    pub fn set_previous_mode(&mut self) {
        if self.current_mode_index > 0 {
            self.current_mode_index -= 1;
        } else {
            self.current_mode_index = self.modes.len() - 1;
        }
    }
}
