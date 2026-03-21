extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct SpeechSwitchInput {
    enabled: bool,
    commands: Vec<String>,
    history: Vec<String>,
    max_history_size: usize,
}

impl SpeechSwitchInput {
    pub fn new(max_history_size: usize) -> Self {
        SpeechSwitchInput {
            enabled: false,
            commands: Vec::new(),
            history: Vec::new(),
            max_history_size,
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
        if self.commands.len() < self.max_history_size {
            self.commands.push(command);
        }
    }

    pub fn get_commands(&self) -> &Vec<String> {
        &self.commands
    }

    pub fn clear_commands(&mut self) {
        self.commands.clear();
    }

    pub fn add_to_history(&mut self, input: String) {
        if self.history.len() >= self.max_history_size {
            self.history.remove(0);
        }
        self.history.push(input);
    }

    pub fn get_history(&self) -> &Vec<String> {
        &self.history
    }

    pub fn clear_history(&mut self) {
        self.history.clear();
    }
}
