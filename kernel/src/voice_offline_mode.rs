extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct VoiceOfflineMode {
    enabled: bool,
    commands: Vec<String>,
    history: Vec<String>,
}

impl VoiceOfflineMode {
    pub fn new() -> Self {
        VoiceOfflineMode {
            enabled: false,
            commands: Vec::new(),
            history: Vec::new(),
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn add_command(&mut self, command: String) {
        self.commands.push(command);
    }

    pub fn get_commands(&self) -> &Vec<String> {
        &self.commands
    }

    pub fn execute_command(&mut self, command: &str) -> Option<&String> {
        if !self.enabled {
            return None;
        }
        for cmd in &self.commands {
            if cmd == command {
                self.history.push(command.to_string());
                return Some(cmd);
            }
        }
        None
    }

    pub fn get_history(&self) -> &Vec<String> {
        &self.history
    }
}
