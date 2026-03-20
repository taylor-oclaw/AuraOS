extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AITokenizerAbstract {
    tokens: Vec<String>,
}

impl AITokenizerAbstract {
    pub fn new() -> Self {
        AITokenizerAbstract {
            tokens: Vec::new(),
        }
    }

    pub fn add_token(&mut self, token: String) {
        self.tokens.push(token);
    }

    pub fn remove_token(&mut self, index: usize) -> Option<String> {
        if index < self.tokens.len() {
            Some(self.tokens.remove(index))
        } else {
            None
        }
    }

    pub fn get_token(&self, index: usize) -> Option<&String> {
        self.tokens.get(index)
    }

    pub fn token_count(&self) -> usize {
        self.tokens.len()
    }

    pub fn clear_tokens(&mut self) {
        self.tokens.clear();
    }
}
