extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct SpeechPhonemeRTL {
    phonemes: Vec<String>,
}

impl SpeechPhonemeRTL {
    pub fn new() -> Self {
        SpeechPhonemeRTL {
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

    pub fn count_phonemes(&self) -> usize {
        self.phonemes.len()
    }

    pub fn reverse_phonemes(&mut self) {
        self.phonemes.reverse();
    }
}
