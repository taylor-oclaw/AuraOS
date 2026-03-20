extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_ffi_init() {
    // Initialize the module
}

pub extern "C" fn rust_ffi_exit() {
    // Cleanup the module
}

pub struct AuraTerminalV2 {
    buffer: Vec<u8>,
    cursor_position: usize,
    history: Vec<String>,
    current_input: String,
}

impl AuraTerminalV2 {
    pub fn new() -> Self {
        AuraTerminalV2 {
            buffer: Vec::new(),
            cursor_position: 0,
            history: Vec::new(),
            current_input: String::new(),
        }
    }

    pub fn write(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(data);
        self.cursor_position += data.len();
    }

    pub fn read_line(&self) -> &str {
        core::str::from_utf8(&self.buffer).unwrap_or("")
    }

    pub fn add_to_history(&mut self) {
        if !self.current_input.is_empty() {
            self.history.push(self.current_input.clone());
            self.current_input.clear();
        }
    }

    pub fn get_previous_command(&mut self) -> Option<&str> {
        if let Some(command) = self.history.pop() {
            self.current_input = command;
            Some(&self.current_input)
        } else {
            None
        }
    }

    pub fn clear_buffer(&mut self) {
        self.buffer.clear();
        self.cursor_position = 0;
    }
}
