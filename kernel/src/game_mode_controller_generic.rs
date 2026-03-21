extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub struct GameModeController {
    current_mode: String,
    available_modes: Vec<String>,
}

impl GameModeController {
    pub fn new() -> Self {
        let modes = vec![
            String::from("Survival"),
            String::from("Creative"),
            String::from("Adventure"),
            String::from("Spectator"),
            String::from("Hardcore"),
        ];
        GameModeController {
            current_mode: modes[0].clone(),
            available_modes: modes,
        }
    }

    pub fn get_current_mode(&self) -> &str {
        &self.current_mode
    }

    pub fn set_mode(&mut self, mode: &str) -> Result<(), String> {
        if self.available_modes.contains(&String::from(mode)) {
            self.current_mode = String::from(mode);
            Ok(())
        } else {
            Err(String::from("Mode not available"))
        }
    }

    pub fn list_available_modes(&self) -> Vec<&str> {
        self.available_modes.iter().map(|m| m.as_str()).collect()
    }

    pub fn is_mode_available(&self, mode: &str) -> bool {
        self.available_modes.contains(&String::from(mode))
    }

    pub fn switch_to_next_mode(&mut self) {
        let current_index = self.available_modes
            .iter()
            .position(|m| m == &self.current_mode)
            .unwrap_or(0);
        let next_index = (current_index + 1) % self.available_modes.len();
        self.current_mode = self.available_modes[next_index].clone();
    }
}
