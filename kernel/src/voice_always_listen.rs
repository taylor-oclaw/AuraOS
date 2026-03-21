extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    let mut listener = VoiceAlwaysListen::new();
    listener.start_listening();
}

pub struct VoiceAlwaysListen {
    listening: bool,
    commands: Vec<String>,
    history: Vec<String>,
}

impl VoiceAlwaysListen {
    pub fn new() -> Self {
        VoiceAlwaysListen {
            listening: false,
            commands: Vec::new(),
            history: Vec::new(),
        }
    }

    pub fn start_listening(&mut self) {
        self.listening = true;
    }

    pub fn stop_listening(&mut self) {
        self.listening = false;
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

    pub fn get_history(&self) -> &Vec<String> {
        &self.history
    }
}
