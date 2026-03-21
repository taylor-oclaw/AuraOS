extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct SpeechFallbackChain {
    fallbacks: Vec<String>,
}

impl SpeechFallbackChain {
    pub fn new() -> Self {
        SpeechFallbackChain {
            fallbacks: Vec::new(),
        }
    }

    pub fn add_fallback(&mut self, fallback: String) {
        self.fallbacks.push(fallback);
    }

    pub fn remove_fallback(&mut self, index: usize) -> Option<String> {
        if index < self.fallbacks.len() {
            Some(self.fallbacks.remove(index))
        } else {
            None
        }
    }

    pub fn get_fallback(&self, index: usize) -> Option<&String> {
        self.fallbacks.get(index)
    }

    pub fn list_fallbacks(&self) -> &[String] {
        &self.fallbacks
    }

    pub fn clear_fallbacks(&mut self) {
        self.fallbacks.clear();
    }
}
