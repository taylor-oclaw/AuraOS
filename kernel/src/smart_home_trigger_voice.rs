extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct SmartHomeTriggerVoice {
    commands: Vec<String>,
    enabled: bool,
}

impl SmartHomeTriggerVoice {
    pub fn new() -> Self {
        SmartHomeTriggerVoice {
            commands: Vec::new(),
            enabled: true,
        }
    }

    pub fn add_command(&mut self, command: String) {
        if !self.commands.contains(&command) {
            self.commands.push(command);
        }
    }

    pub fn remove_command(&mut self, command: &str) -> bool {
        let index = self.commands.iter().position(|c| c == command);
        match index {
            Some(i) => {
                self.commands.remove(i);
                true
            }
            None => false,
        }
    }

    pub fn list_commands(&self) -> Vec<String> {
        self.commands.clone()
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn toggle_enable(&mut self) {
        self.enabled = !self.enabled;
    }
}
