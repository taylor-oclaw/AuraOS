extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn speech_stutter_patience_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn speech_stutter_patience_exit() {
    // Cleanup logic for the module
}

pub struct SpeechStutterPatience {
    words: Vec<String>,
    patience_level: usize,
}

impl SpeechStutterPatience {
    pub fn new(words: Vec<String>, patience_level: usize) -> Self {
        SpeechStutterPatience { words, patience_level }
    }

    pub fn add_word(&mut self, word: String) {
        self.words.push(word);
    }

    pub fn remove_word(&mut self, index: usize) -> Option<String> {
        if index < self.words.len() {
            Some(self.words.remove(index))
        } else {
            None
        }
    }

    pub fn get_word(&self, index: usize) -> Option<&String> {
        self.words.get(index)
    }

    pub fn stutter(&self, word_index: usize, repeat_count: usize) -> Option<String> {
        if let Some(word) = self.get_word(word_index) {
            let mut stuttered_word = String::new();
            for _ in 0..repeat_count {
                stuttered_word.push_str(word);
            }
            Some(stuttered_word)
        } else {
            None
        }
    }

    pub fn patience_level(&self) -> usize {
        self.patience_level
    }
}
