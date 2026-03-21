extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct SpeechPhonemeThToF {
    phonemes: Vec<String>,
}

impl SpeechPhonemeThToF {
    pub fn new() -> Self {
        SpeechPhonemeThToF {
            phonemes: Vec::new(),
        }
    }

    pub fn add_phoneme(&mut self, phoneme: String) {
        self.phonemes.push(phoneme);
    }

    pub fn get_phoneme_count(&self) -> usize {
        self.phonemes.len()
    }

    pub fn replace_th_with_f(&mut self) {
        for phoneme in &mut self.phonemes {
            if phoneme.contains("th") {
                *phoneme = phoneme.replace("th", "f");
            }
        }
    }

    pub fn get_phonemes(&self) -> Vec<String> {
        self.phonemes.clone()
    }

    pub fn clear_phonemes(&mut self) {
        self.phonemes.clear();
    }
}
