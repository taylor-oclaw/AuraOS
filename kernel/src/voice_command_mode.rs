extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct VoiceCommandMode {
    commands: Vec<String>,
    active: bool,
}

impl VoiceCommandMode {
    pub fn new() -> Self {
        VoiceCommandMode {
            commands: Vec::new(),
            active: false,
        }
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn add_command(&mut self, command: String) {
        self.commands.push(command);
    }

    pub fn remove_command(&mut self, command: &str) -> bool {
        if let Some(index) = self.commands.iter().position(|c| c == command) {
            self.commands.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_commands(&self) -> Vec<String> {
        self.commands.clone()
    }
}
