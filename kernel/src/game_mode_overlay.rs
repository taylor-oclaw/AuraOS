extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct GameModeOverlay {
    active: bool,
    modes: Vec<String>,
    current_mode_index: usize,
}

impl GameModeOverlay {
    pub fn new() -> Self {
        GameModeOverlay {
            active: false,
            modes: Vec::new(),
            current_mode_index: 0,
        }
    }

    pub fn activate(&mut self) {
        if !self.modes.is_empty() {
            self.active = true;
        }
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn add_mode(&mut self, mode_name: &str) {
        self.modes.push(String::from(mode_name));
        if self.modes.len() == 1 {
            self.current_mode_index = 0;
            self.activate();
        }
    }

    pub fn remove_mode(&mut self, mode_name: &str) {
        if let Some(index) = self.modes.iter().position(|m| m == mode_name) {
            self.modes.remove(index);
            if index == self.current_mode_index && !self.modes.is_empty() {
                self.current_mode_index %= self.modes.len();
            }
            if self.modes.is_empty() {
                self.deactivate();
            }
        }
    }

    pub fn switch_to_next_mode(&mut self) {
        if self.active && !self.modes.is_empty() {
            self.current_mode_index = (self.current_mode_index + 1) % self.modes.len();
        }
    }

    pub fn current_mode(&self) -> Option<&String> {
        if self.active {
            Some(&self.modes[self.current_mode_index])
        } else {
            None
        }
    }
}
