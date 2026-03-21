extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct GameModeLauncher {
    modes: Vec<String>,
    current_mode: Option<usize>,
}

impl GameModeLauncher {
    pub fn new() -> Self {
        GameModeLauncher {
            modes: Vec::new(),
            current_mode: None,
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

    pub fn list_modes(&self) -> Vec<String> {
        self.modes.clone()
    }

    pub fn start_mode(&mut self, mode_name: &str) -> bool {
        if let Some(index) = self.modes.iter().position(|m| m == mode_name) {
            self.current_mode = Some(index);
            true
        } else {
            false
        }
    }

    pub fn stop_current_mode(&mut self) {
        self.current_mode = None;
    }

    pub fn current_mode_name(&self) -> Option<String> {
        self.current_mode.map(|index| self.modes[index].clone())
    }
}
