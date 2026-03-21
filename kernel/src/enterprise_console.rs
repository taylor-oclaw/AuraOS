extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct EnterpriseConsole {
    commands: Vec<String>,
    history: Vec<String>,
}

impl EnterpriseConsole {
    pub fn new() -> Self {
        EnterpriseConsole {
            commands: Vec::new(),
            history: Vec::new(),
        }
    }

    pub fn add_command(&mut self, command: String) {
        self.commands.push(command);
    }

    pub fn execute_command(&mut self, command: &str) -> Option<String> {
        if let Some(index) = self.commands.iter().position(|c| c == command) {
            let result = format!("Executing command: {}", self.commands[index]);
            self.history.push(result.clone());
            Some(result)
        } else {
            None
        }
    }

    pub fn list_commands(&self) -> Vec<String> {
        self.commands.clone()
    }

    pub fn get_history(&self) -> Vec<String> {
        self.history.clone()
    }

    pub fn clear_history(&mut self) {
        self.history.clear();
    }
}
