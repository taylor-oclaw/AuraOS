extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

pub struct VoiceUndoRedo {
    history: Vec<String>,
    future: Vec<String>,
}

impl VoiceUndoRedo {
    pub fn new() -> Self {
        VoiceUndoRedo {
            history: Vec::new(),
            future: Vec::new(),
        }
    }

    pub fn add_command(&mut self, command: String) {
        self.history.push(command);
        self.future.clear();
    }

    pub fn undo(&mut self) -> Option<String> {
        if let Some(last_command) = self.history.pop() {
            self.future.push(last_command.clone());
            Some(last_command)
        } else {
            None
        }
    }

    pub fn redo(&mut self) -> Option<String> {
        if let Some(next_command) = self.future.pop() {
            self.history.push(next_command.clone());
            Some(next_command)
        } else {
            None
        }
    }

    pub fn get_history(&self) -> Vec<String> {
        self.history.clone()
    }

    pub fn get_future(&self) -> Vec<String> {
        self.future.clone()
    }
}
