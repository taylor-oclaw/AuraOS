extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn speech_lisp_lateral_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn speech_lisp_lateral_exit() {
    // Cleanup logic for the module
}

pub struct SpeechLispLateral {
    commands: Vec<String>,
    history: Vec<String>,
    max_history_size: usize,
}

impl SpeechLispLateral {
    pub fn new(max_history_size: usize) -> Self {
        SpeechLispLateral {
            commands: Vec::new(),
            history: Vec::new(),
            max_history_size,
        }
    }

    pub fn add_command(&mut self, command: String) {
        self.commands.push(command);
    }

    pub fn execute_command(&mut self, index: usize) -> Option<String> {
        if let Some(command) = self.commands.get(index).cloned() {
            self.history.push(command.clone());
            if self.history.len() > self.max_history_size {
                self.history.remove(0);
            }
            Some(command)
        } else {
            None
        }
    }

    pub fn get_command_count(&self) -> usize {
        self.commands.len()
    }

    pub fn get_history(&self) -> Vec<String> {
        self.history.clone()
    }

    pub fn clear_commands(&mut self) {
        self.commands.clear();
    }
}
