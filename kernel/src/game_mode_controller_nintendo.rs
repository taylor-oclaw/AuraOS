extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub struct GameModeControllerNintendo {
    current_mode: String,
    available_modes: Vec<String>,
}

impl GameModeControllerNintendo {
    pub fn new() -> Self {
        let modes = vec![
            String::from("Standard"),
            String::from("Adventure"),
            String::from("Challenge"),
            String::from("Time Trial"),
            String::from("Survival"),
        ];
        GameModeControllerNintendo {
            current_mode: modes[0].clone(),
            available_modes: modes,
        }
    }

    pub fn get_current_mode(&self) -> &str {
        &self.current_mode
    }

    pub fn set_mode(&mut self, mode: &str) -> Result<(), String> {
        if self.available_modes.contains(&mode.to_string()) {
            self.current_mode = mode.to_string();
            Ok(())
        } else {
            Err(String::from("info"))
        }
    }

    pub fn list_available_modes(&self) -> Vec<&str> {
        self.available_modes.iter().map(|m| m.as_str()).collect()
    }

    pub fn add_mode(&mut self, mode: &str) -> Result<(), String> {
        if !self.available_modes.contains(&mode.to_string()) {
            self.available_modes.push(mode.to_string());
            Ok(())
        } else {
            Err(String::from("info"))
        }
    }

    pub fn remove_mode(&mut self, mode: &str) -> Result<(), String> {
        if let Some(index) = self.available_modes.iter().position(|m| m == mode) {
            self.available_modes.remove(index);
            if self.current_mode == mode {
                self.current_mode = self.available_modes.get(0).cloned().unwrap_or_default();
            }
            Ok(())
        } else {
            Err(String::from("info"))
        }
    }
}
