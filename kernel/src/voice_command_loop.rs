extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct VoiceCommandLoop {
    commands: Vec<String>,
    current_index: usize,
}

impl VoiceCommandLoop {
    pub fn new() -> Self {
        VoiceCommandLoop {
            commands: Vec::new(),
            current_index: 0,
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

    pub fn get_current_command(&self) -> Option<&String> {
        if self.current_index < self.commands.len() {
            Some(&self.commands[self.current_index])
        } else {
            None
        }
    }

    pub fn next_command(&mut self) -> Option<&String> {
        if self.current_index + 1 < self.commands.len() {
            self.current_index += 1;
            Some(&self.commands[self.current_index])
        } else {
            None
        }
    }

    pub fn previous_command(&mut self) -> Option<&String> {
        if self.current_index > 0 {
            self.current_index -= 1;
            Some(&self.commands[self.current_index])
        } else {
            None
        }
    }
}
