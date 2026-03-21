extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct SpeechContextCorrect {
    context: Vec<String>,
}

impl SpeechContextCorrect {
    pub fn new() -> Self {
        SpeechContextCorrect {
            context: Vec::new(),
        }
    }

    pub fn add_context(&mut self, word: &str) {
        self.context.push(word.to_string());
    }

    pub fn remove_context(&mut self, index: usize) -> Option<String> {
        if index < self.context.len() {
            Some(self.context.remove(index))
        } else {
            None
        }
    }

    pub fn get_context(&self, index: usize) -> Option<&String> {
        self.context.get(index)
    }

    pub fn clear_context(&mut self) {
        self.context.clear();
    }

    pub fn correct_speech(&self, input: &str) -> String {
        let mut corrected = String::new();
        for word in input.split_whitespace() {
            if let Some(context_word) = self.get_context(0) {
                if context_word == word {
                    corrected.push_str("corrected_");
                }
            }
            corrected.push_str(word);
            corrected.push(' ');
        }
        corrected.trim_end().to_string()
    }
}
