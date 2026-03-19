extern crate alloc;

use alloc::vec::Vec;
use alloc::string::String;

pub struct ClipboardEntry {
    pub content: Vec<u8>,
    pub mime_type: String,
    pub timestamp: u64,
}

pub struct Clipboard {
    history: Vec<ClipboardEntry>,
    max_entries: usize,
}

impl Clipboard {
    pub fn new() -> Self {
        Self { history: Vec::new(), max_entries: 50 }
    }

    pub fn copy(&mut self, data: &[u8], mime: &str, timestamp: u64) {
        if self.history.len() >= self.max_entries {
            self.history.remove(0);
        }
        self.history.push(ClipboardEntry {
            content: Vec::from(data),
            mime_type: String::from(mime),
            timestamp,
        });
    }

    pub fn paste(&self) -> Option<&ClipboardEntry> {
        self.history.last()
    }

    pub fn paste_at(&self, index: usize) -> Option<&ClipboardEntry> {
        self.history.get(index)
    }

    pub fn clear(&mut self) {
        self.history.clear();
    }

    pub fn history_len(&self) -> usize {
        self.history.len()
    }

    pub fn paste_text(&self) -> Option<&str> {
        self.history.last().and_then(|e| core::str::from_utf8(&e.content).ok())
    }
}
