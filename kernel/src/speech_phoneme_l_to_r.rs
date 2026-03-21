extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn speech_phoneme_l_to_r_init() {
    // Initialization code for the module
}

pub extern "C" fn speech_phoneme_l_to_r_exit() {
    // Cleanup code for the module
}

pub struct SpeechPhonemeLTR {
    phonemes: Vec<String>,
}

impl SpeechPhonemeLTR {
    pub fn new() -> Self {
        SpeechPhonemeLTR {
            phonemes: Vec::new(),
        }
    }

    pub fn add_phoneme(&mut self, phoneme: String) {
        self.phonemes.push(phoneme);
    }

    pub fn get_phoneme_count(&self) -> usize {
        self.phonemes.len()
    }

    pub fn get_phoneme_at(&self, index: usize) -> Option<&String> {
        self.phonemes.get(index)
    }

    pub fn remove_phoneme_at(&mut self, index: usize) -> Option<String> {
        if index < self.phonemes.len() {
            Some(self.phonemes.remove(index))
        } else {
            None
        }
    }

    pub fn clear_phonemes(&mut self) {
        self.phonemes.clear();
    }
}
