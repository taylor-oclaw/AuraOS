extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct GameModeHaptic {
    mode: String,
    intensity: u8,
    duration: u16,
    patterns: Vec<u8>,
    is_active: bool,
}

impl GameModeHaptic {
    pub fn new(mode: &str, intensity: u8, duration: u16) -> Self {
        GameModeHaptic {
            mode: String::from(mode),
            intensity,
            duration,
            patterns: Vec::new(),
            is_active: false,
        }
    }

    pub fn set_mode(&mut self, mode: &str) {
        self.mode = String::from(mode);
    }

    pub fn get_mode(&self) -> &String {
        &self.mode
    }

    pub fn set_intensity(&mut self, intensity: u8) {
        if intensity <= 100 {
            self.intensity = intensity;
        }
    }

    pub fn get_intensity(&self) -> u8 {
        self.intensity
    }

    pub fn add_pattern(&mut self, pattern: u8) {
        self.patterns.push(pattern);
    }

    pub fn remove_pattern(&mut self, index: usize) -> Option<u8> {
        if index < self.patterns.len() {
            Some(self.patterns.remove(index))
        } else {
            None
        }
    }

    pub fn activate(&mut self) {
        self.is_active = true;
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    pub fn is_activated(&self) -> bool {
        self.is_active
    }

    pub fn set_duration(&mut self, duration: u16) {
        self.duration = duration;
    }

    pub fn get_duration(&self) -> u16 {
        self.duration
    }
}
