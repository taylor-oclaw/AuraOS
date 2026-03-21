extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct SpeechRepeatRequest {
    text: String,
    repeat_count: usize,
    current_repeat: usize,
}

impl SpeechRepeatRequest {
    pub fn new(text: &str, repeat_count: usize) -> Self {
        SpeechRepeatRequest {
            text: String::from(text),
            repeat_count,
            current_repeat: 0,
        }
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, new_text: &str) {
        self.text = String::from(new_text);
    }

    pub fn get_repeat_count(&self) -> usize {
        self.repeat_count
    }

    pub fn set_repeat_count(&mut self, new_count: usize) {
        self.repeat_count = new_count;
    }

    pub fn next_repeat(&mut self) -> Option<&str> {
        if self.current_repeat < self.repeat_count {
            self.current_repeat += 1;
            Some(&self.text)
        } else {
            None
        }
    }

    pub fn reset(&mut self) {
        self.current_repeat = 0;
    }
}
