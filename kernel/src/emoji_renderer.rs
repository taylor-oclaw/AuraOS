extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct EmojiRenderer {
    emojis: Vec<String>,
}

impl EmojiRenderer {
    pub fn new() -> Self {
        EmojiRenderer {
            emojis: Vec::new(),
        }
    }

    pub fn add_emoji(&mut self, emoji: &str) {
        self.emojis.push(String::from(emoji));
    }

    pub fn remove_emoji(&mut self, index: usize) -> Option<String> {
        if index < self.emojis.len() {
            Some(self.emojis.remove(index))
        } else {
            None
        }
    }

    pub fn get_emoji(&self, index: usize) -> Option<&String> {
        self.emojis.get(index)
    }

    pub fn list_emojis(&self) -> &Vec<String> {
        &self.emojis
    }

    pub fn clear_emojis(&mut self) {
        self.emojis.clear();
    }
}
