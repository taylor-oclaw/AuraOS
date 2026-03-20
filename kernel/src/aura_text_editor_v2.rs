extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_ffi_init() {
    // Initialize the module if needed
}

pub extern "C" fn rust_ffi_exit() {
    // Clean up resources if needed
}

pub struct AuraTextEditorV2 {
    content: String,
    cursor_position: usize,
    history: Vec<String>,
}

impl AuraTextEditorV2 {
    pub fn new() -> Self {
        AuraTextEditorV2 {
            content: String::new(),
            cursor_position: 0,
            history: Vec::new(),
        }
    }

    pub fn insert(&mut self, text: &str) {
        let mut new_content = self.content.clone();
        new_content.insert_str(self.cursor_position, text);
        self.history.push(new_content.clone());
        self.content = new_content;
        self.move_cursor(text.len());
    }

    pub fn delete(&mut self) {
        if self.cursor_position > 0 && !self.content.is_empty() {
            let mut new_content = self.content.clone();
            new_content.remove(self.cursor_position - 1);
            self.history.push(new_content.clone());
            self.content = new_content;
            self.move_cursor(-1);
        }
    }

    pub fn move_cursor(&mut self, offset: isize) {
        let new_position = (self.cursor_position as isize + offset).max(0) as usize;
        self.cursor_position = new_position.min(self.content.len());
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }

    pub fn undo(&mut self) {
        if !self.history.is_empty() {
            let last_state = self.history.pop().unwrap();
            self.content = last_state;
            self.cursor_position = self.content.len();
        }
    }
}
