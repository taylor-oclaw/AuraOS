extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

pub struct ClipboardMgr {
    content: String,
    history: Vec<String>,
}

impl ClipboardMgr {
    pub fn new() -> Self {
        ClipboardMgr {
            content: String::new(),
            history: Vec::new(),
        }
    }

    pub fn copy(&mut self, text: &str) {
        if !self.content.is_empty() {
            self.history.push(self.content.clone());
        }
        self.content = text;
    }

    pub fn paste(&self) -> String {
        self.content.clone()
    }

    pub fn clear(&mut self) {
        self.content.clear();
    }

    pub fn undo(&mut self) -> Option<String> {
        if let Some(last_content) = self.history.pop() {
            self.content = last_content;
            Some(self.content.clone())
        } else {
            None
        }
    }

    pub fn history_size(&self) -> usize {
        self.history.len()
    }
}
