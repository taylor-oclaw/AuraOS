extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct SpeechVowelDistortion {
    vowels: Vec<char>,
    distortion_map: [char; 10],
}

impl SpeechVowelDistortion {
    pub fn new() -> Self {
        SpeechVowelDistortion {
            vowels: vec!['a', 'e', 'i', 'o', 'u'],
            distortion_map: [
                'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm',
            ],
        }
    }

    pub fn add_vowel(&mut self, vowel: char) {
        if !self.vowels.contains(&vowel) && vowel.is_ascii_lowercase() {
            self.vowels.push(vowel);
        }
    }

    pub fn remove_vowel(&mut self, vowel: char) {
        if let Some(index) = self.vowels.iter().position(|&v| v == vowel) {
            self.vowels.remove(index);
        }
    }

    pub fn distort_text(&self, text: &str) -> String {
        let mut distorted_text = String::new();
        for c in text.chars() {
            if self.vowels.contains(&c) {
                let index = (c as u8 - b'a') % self.distortion_map.len() as u8;
                distorted_text.push(self.distortion_map[index as usize]);
            } else {
                distorted_text.push(c);
            }
        }
        distorted_text
    }

    pub fn is_vowel(&self, c: char) -> bool {
        self.vowels.contains(&c)
    }

    pub fn get_vowels(&self) -> Vec<char> {
        self.vowels.clone()
    }
}
