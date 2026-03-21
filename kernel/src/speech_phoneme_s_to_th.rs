extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct SpeechPhonemeSToTH {
    phonemes: Vec<String>,
}

impl SpeechPhonemeSToTH {
    pub fn new() -> Self {
        SpeechPhonemeSToTH {
            phonemes: Vec::new(),
        }
    }

    pub fn add_phoneme(&mut self, phoneme: String) {
        self.phonemes.push(phoneme);
    }

    pub fn remove_phoneme(&mut self, index: usize) -> Option<String> {
        if index < self.phonemes.len() {
            Some(self.phonemes.remove(index))
        } else {
            None
        }
    }

    pub fn get_phoneme(&self, index: usize) -> Option<&String> {
        self.phonemes.get(index)
    }

    pub fn replace_phoneme(&mut self, index: usize, new_phoneme: String) -> bool {
        if index < self.phonemes.len() {
            self.phonemes[index] = new_phoneme;
            true
        } else {
            false
        }
    }

    pub fn list_phonemes(&self) -> &[String] {
        &self.phonemes
    }
}
