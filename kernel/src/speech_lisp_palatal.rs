extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct SpeechLispPalatal {
    commands: Vec<String>,
}

impl SpeechLispPalatal {
    pub fn new() -> Self {
        SpeechLispPalatal {
            commands: Vec::new(),
        }
    }

    pub fn add_command(&mut self, command: String) {
        self.commands.push(command);
    }

    pub fn remove_command(&mut self, index: usize) -> Option<String> {
        if index < self.commands.len() {
            Some(self.commands.remove(index))
        } else {
            None
        }
    }

    pub fn get_command(&self, index: usize) -> Option<&String> {
        self.commands.get(index)
    }

    pub fn list_commands(&self) -> &[String] {
        &self.commands
    }

    pub fn execute_command(&self, command: &str) -> bool {
        self.commands.contains(&String::from(command))
    }
}
