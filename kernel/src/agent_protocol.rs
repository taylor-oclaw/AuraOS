extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct AgentProtocol {
    commands: Vec<String>,
}

impl AgentProtocol {
    pub fn new() -> Self {
        AgentProtocol {
            commands: Vec::new(),
        }
    }

    pub fn add_command(&mut self, command: String) {
        self.commands.push(command);
    }

    pub fn get_commands(&self) -> &Vec<String> {
        &self.commands
    }

    pub fn execute_command(&self, index: usize) -> Option<&String> {
        self.commands.get(index)
    }

    pub fn remove_command(&mut self, index: usize) -> Option<String> {
        if index < self.commands.len() {
            Some(self.commands.remove(index))
        } else {
            None
        }
    }

    pub fn clear_commands(&mut self) {
        self.commands.clear();
    }
}
