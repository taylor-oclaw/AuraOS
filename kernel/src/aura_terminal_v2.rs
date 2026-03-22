extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_ffi_init() {
    // Initialize the module
}

#[no_mangle]
pub extern "C" fn rust_ffi_exit() {
    // Clean up the module
}

pub struct AuraTerminalV2 {
    buffer: Vec<u8>,
    cursor_position: usize,
    history: Vec<String>,
    current_command: String,
}

impl AuraTerminalV2 {
    pub fn new() -> Self {
        AuraTerminalV2 {
            buffer: Vec::new(),
            cursor_position: 0,
            history: Vec::new(),
            current_command: String::new(),
        }
    }

    pub fn add_char(&mut self, c: char) {
        if self.cursor_position < self.current_command.len() {
            self.current_command.insert(self.cursor_position, c);
            self.buffer.push(c as u8);
            self.move_cursor_right();
        } else {
            self.current_command.push(c);
            self.buffer.push(c as u8);
        }
    }

    pub fn backspace(&mut self) {
        if !self.current_command.is_empty() && self.cursor_position > 0 {
            self.cursor_position -= 1;
            self.current_command.remove(self.cursor_position);
            self.buffer.pop();
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_position < self.current_command.len() {
            self.cursor_position += 1;
        }
    }

    pub fn execute_command(&mut self) -> String {
        let command = self.current_command.clone();
        self.history.push(command.clone());
        self.current_command.clear();
        self.buffer.clear();
        self.cursor_position = 0;
        // Simulate command execution
        format!("Executed: {}", command)
    }
}