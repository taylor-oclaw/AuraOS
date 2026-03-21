extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn speech_nasal_speech_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn speech_nasal_speech_exit() {
    // Cleanup logic for the module
}

pub struct SpeechNasalSpeech {
    words: Vec<String>,
    current_index: usize,
}

impl SpeechNasalSpeech {
    pub fn new(words: Vec<&str>) -> Self {
        let word_vec = words.into_iter().map(|w| w.to_string()).collect();
        SpeechNasalSpeech {
            words: word_vec,
            current_index: 0,
        }
    }

    pub fn add_word(&mut self, word: &str) {
        self.words.push(word.to_string());
    }

    pub fn remove_word(&mut self, index: usize) -> Option<String> {
        if index < self.words.len() {
            Some(self.words.remove(index))
        } else {
            None
        }
    }

    pub fn get_current_word(&self) -> Option<&String> {
        if self.current_index < self.words.len() {
            Some(&self.words[self.current_index])
        } else {
            None
        }
    }

    pub fn next_word(&mut self) -> Option<&String> {
        if self.current_index < self.words.len() {
            let current = &self.words[self.current_index];
            self.current_index += 1;
            Some(current)
        } else {
            None
        }
    }

    pub fn reset(&mut self) {
        self.current_index = 0;
    }
}
