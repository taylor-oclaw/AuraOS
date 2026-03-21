extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct SpeechStutterWaitMode {
    words: Vec<String>,
    current_index: usize,
    wait_time: u32,
}

impl SpeechStutterWaitMode {
    pub fn new(words: Vec<String>, wait_time: u32) -> Self {
        SpeechStutterWaitMode {
            words,
            current_index: 0,
            wait_time,
        }
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

    pub fn get_current_word(&self) -> Option<&String> {
        if self.current_index < self.words.len() {
            Some(&self.words[self.current_index])
        } else {
            None
        }
    }

    pub fn next_word(&mut self) -> Option<&String> {
        if self.current_index < self.words.len() {
            let current_word = &self.words[self.current_index];
            self.current_index += 1;
            Some(current_word)
        } else {
            None
        }
    }

    pub fn reset(&mut self) {
        self.current_index = 0;
    }
}
