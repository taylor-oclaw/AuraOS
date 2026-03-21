extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn speech_toddler_adapt_init() {
    // Initialization logic for the module
}

pub extern "C" fn speech_toddler_adapt_exit() {
    // Cleanup logic for the module
}

pub struct SpeechToddlerAdapt {
    vocabulary: Vec<String>,
    learning_rate: f32,
    max_vocabulary_size: usize,
}

impl SpeechToddlerAdapt {
    pub fn new(max_vocabulary_size: usize, learning_rate: f32) -> Self {
        SpeechToddlerAdapt {
            vocabulary: Vec::new(),
            learning_rate,
            max_vocabulary_size,
        }
    }

    pub fn add_word(&mut self, word: String) {
        if self.vocabulary.len() < self.max_vocabulary_size {
            self.vocabulary.push(word);
        }
    }

    pub fn remove_word(&mut self, word: &str) -> bool {
        if let Some(index) = self.vocabulary.iter().position(|w| w == word) {
            self.vocabulary.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_vocabulary_size(&self) -> usize {
        self.vocabulary.len()
    }

    pub fn update_learning_rate(&mut self, new_rate: f32) {
        self.learning_rate = new_rate;
    }

    pub fn contains_word(&self, word: &str) -> bool {
        self.vocabulary.contains(&String::from(word))
    }
}
