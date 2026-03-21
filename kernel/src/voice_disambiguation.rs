extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct VoiceDisambiguation {
    // Example fields, replace with actual logic
    commands: Vec<String>,
    current_command_index: usize,
}

impl VoiceDisambiguation {
    pub fn new() -> Self {
        VoiceDisambiguation {
            commands: Vec::new(),
            current_command_index: 0,
        }
    }

    pub fn add_command(&mut self, command: String) {
        self.commands.push(command);
    }

    pub fn get_current_command(&self) -> Option<&String> {
        if self.current_command_index < self.commands.len() {
            Some(&self.commands[self.current_command_index])
        } else {
            None
        }
    }

    pub fn next_command(&mut self) {
        if self.current_command_index < self.commands.len() - 1 {
            self.current_command_index += 1;
        }
    }

    pub fn previous_command(&mut self) {
        if self.current_command_index > 0 {
            self.current_command_index -= 1;
        }
    }

    pub fn remove_current_command(&mut self) -> Option<String> {
        if self.current_command_index < self.commands.len() {
            Some(self.commands.remove(self.current_command_index))
        } else {
            None
        }
    }
}
