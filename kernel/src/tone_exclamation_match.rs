extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct TonePunctuationStyle {
    tone: String,
    punctuation: String,
    style: String,
}

impl TonePunctuationStyle {
    pub fn new(tone: &str, punctuation: &str, style: &str) -> Self {
        TonePunctuationStyle {
            tone: String::from(tone),
            punctuation: String::from(punctuation),
            style: String::from(style),
        }
    }

    pub fn get_tone(&self) -> &str {
        &self.tone
    }

    pub fn set_tone(&mut self, tone: &str) {
        self.tone = String::from(tone);
    }

    pub fn get_punctuation(&self) -> &str {
        &self.punctuation
    }

    pub fn set_punctuation(&mut self, punctuation: &str) {
        self.punctuation = String::from(punctuation);
    }

    pub fn get_style(&self) -> &str {
        &self.style
    }

    pub fn set_style(&mut self, style: &str) {
        self.style = String::from(style);
    }

    pub fn apply_to_text(&self, text: &str) -> String {
        let mut result = String::new();
        for c in text.chars() {
            if c.is_alphabetic() {
                result.push_str(&self.tone);
                result.push(c);
                result.push_str(&self.punctuation);
            } else {
                result.push(c);
            }
        }
        result
    }

    pub fn remove_style(&self, text: &str) -> String {
        let mut result = String::new();
        let tone_len = self.tone.len();
        let punctuation_len = self.punctuation.len();

        let mut chars = text.chars().peekable();
        while let Some(c) = chars.next() {
            if c.is_alphabetic() && chars.peek().is_some() && chars.peek().unwrap().is_alphabetic() {
                result.push(c);
                for _ in 0..tone_len + punctuation_len {
                    chars.next();
                }
            } else {
                result.push(c);
            }
        }

        result
    }
}
