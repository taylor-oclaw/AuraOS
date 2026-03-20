extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_ffi_init() {
    // Initialize the aura_screensaver module
}

#[no_mangle]
pub extern "C" fn rust_ffi_exit() {
    // Clean up the aura_screensaver module
}

pub struct AuraScreensaver {
    patterns: Vec<String>,
    current_pattern_index: usize,
    screen_width: usize,
    screen_height: usize,
    frame_rate: usize,
}

impl AuraScreensaver {
    pub fn new(patterns: Vec<String>, screen_width: usize, screen_height: usize, frame_rate: usize) -> Self {
        AuraScreensaver {
            patterns,
            current_pattern_index: 0,
            screen_width,
            screen_height,
            frame_rate,
        }
    }

    pub fn next_pattern(&mut self) {
        self.current_pattern_index = (self.current_pattern_index + 1) % self.patterns.len();
    }

    pub fn previous_pattern(&mut self) {
        if self.current_pattern_index == 0 {
            self.current_pattern_index = self.patterns.len() - 1;
        } else {
            self.current_pattern_index -= 1;
        }
    }

    pub fn get_current_pattern(&self) -> &String {
        &self.patterns[self.current_pattern_index]
    }

    pub fn set_frame_rate(&mut self, frame_rate: usize) {
        if frame_rate > 0 {
            self.frame_rate = frame_rate;
        }
    }

    pub fn get_frame_rate(&self) -> usize {
        self.frame_rate
    }
}
