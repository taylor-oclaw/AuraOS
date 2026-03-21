extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn speech_hypernasal_init() {
    // Initialization logic for the module
}

pub extern "C" fn speech_hypernasal_exit() {
    // Cleanup logic for the module
}

pub struct SpeechHypernasal {
    phrases: Vec<String>,
    current_phrase_index: usize,
}

impl SpeechHypernasal {
    pub fn new(phrases: Vec<&str>) -> Self {
        let phrases = phrases.into_iter().map(|s| String::from(s)).collect();
        SpeechHypernasal {
            phrases,
            current_phrase_index: 0,
        }
    }

    pub fn add_phrase(&mut self, phrase: &str) {
        self.phrases.push(String::from(phrase));
    }

    pub fn remove_phrase(&mut self, index: usize) -> Option<String> {
        if index < self.phrases.len() {
            Some(self.phrases.remove(index))
        } else {
            None
        }
    }

    pub fn get_current_phrase(&self) -> &str {
        if let Some(phrase) = self.phrases.get(self.current_phrase_index) {
            phrase.as_str()
        } else {
            ""
        }
    }

    pub fn next_phrase(&mut self) -> &str {
        if self.current_phrase_index < self.phrases.len() {
            self.current_phrase_index += 1;
        } else {
            self.current_phrase_index = 0;
        }
        self.get_current_phrase()
    }

    pub fn previous_phrase(&mut self) -> &str {
        if self.current_phrase_index > 0 {
            self.current_phrase_index -= 1;
        } else {
            self.current_phrase_index = self.phrases.len().saturating_sub(1);
        }
        self.get_current_phrase()
    }
}
