extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub enum ClipboardContent {
    Text(String),
    Binary(Vec<u8>),
}

pub struct Clipboard {
    content: Option<ClipboardContent>,
    history: Vec<ClipboardContent>,
    max_history: usize,
}

impl Clipboard {
    pub fn new(max_history: usize) -> Self {
        Self {
            content: None,
            history: Vec::new(),
            max_history,
        }
    }

    pub fn copy_text(&mut self, text: String) {
        if let Some(old) = self.content.take() {
            self.history.push(old);
            if self.history.len() > self.max_history {
                self.history.remove(0);
            }
        }
        self.content = Some(ClipboardContent::Text(text));
    }

    pub fn paste_text(&self) -> Option<&str> {
        match &self.content {
            Some(ClipboardContent::Text(t)) => Some(t.as_str()),
            _ => None,
        }
    }

    pub fn clear(&mut self) {
        self.content = None;
    }

    pub fn history_len(&self) -> usize {
        self.history.len()
    }
}
