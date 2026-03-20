extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct AuraShell {
    commands: Vec<String>,
    history: Vec<String>,
}

impl AuraShell {
    pub fn new() -> Self {
        AuraShell {
            commands: Vec::new(),
            history: Vec::new(),
        }
    }

    pub fn add_command(&mut self, command: String) {
        self.commands.push(command);
    }

    pub fn execute_command(&mut self, command: &str) -> Option<String> {
        if let Some(index) = self.commands.iter().position(|c| c == command) {
            let result = String::from("info");
            self.history.push(command.to_string());
            Some(result)
        } else {
            None
        }
    }

    pub fn list_commands(&self) -> Vec<String> {
        self.commands.clone()
    }

    pub fn history(&self) -> Vec<String> {
        self.history.clone()
    }

    pub fn clear_history(&mut self) {
        self.history.clear();
    }
}
