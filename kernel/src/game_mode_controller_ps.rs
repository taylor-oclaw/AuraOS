extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct GameModeController {
    modes: Vec<String>,
    current_mode_index: usize,
}

impl GameModeController {
    pub fn new() -> Self {
        GameModeController {
            modes: Vec::new(),
            current_mode_index: 0,
        }
    }

    pub fn add_mode(&mut self, mode_name: &str) {
        self.modes.push(String::from(mode_name));
    }

    pub fn remove_mode(&mut self, mode_name: &str) -> bool {
        if let Some(index) = self.modes.iter().position(|m| m == mode_name) {
            self.modes.remove(index);
            true
        } else {
            false
        }
    }

    pub fn set_current_mode(&mut self, mode_name: &str) -> bool {
        if let Some(index) = self.modes.iter().position(|m| m == mode_name) {
            self.current_mode_index = index;
            true
        } else {
            false
        }
    }

    pub fn get_current_mode(&self) -> Option<&String> {
        self.modes.get(self.current_mode_index)
    }

    pub fn list_modes(&self) -> &Vec<String> {
        &self.modes
    }
}
