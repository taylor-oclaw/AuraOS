extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ToneFamilyMode {
    mode_name: String,
    tones: Vec<String>,
    active_tone_index: usize,
}

impl ToneFamilyMode {
    pub fn new(mode_name: &str, initial_tones: &[&str]) -> Self {
        let mut tones = Vec::new();
        for tone in initial_tones {
            tones.push(String::from(*tone));
        }
        ToneFamilyMode {
            mode_name: String::from(mode_name),
            tones,
            active_tone_index: 0,
        }
    }

    pub fn get_mode_name(&self) -> &str {
        &self.mode_name
    }

    pub fn add_tone(&mut self, tone: &str) {
        self.tones.push(String::from(tone));
    }

    pub fn remove_tone(&mut self, index: usize) -> Option<String> {
        if index < self.tones.len() {
            Some(self.tones.remove(index))
        } else {
            None
        }
    }

    pub fn get_active_tone(&self) -> &str {
        &self.tones[self.active_tone_index]
    }

    pub fn set_active_tone(&mut self, index: usize) -> bool {
        if index < self.tones.len() {
            self.active_tone_index = index;
            true
        } else {
            false
        }
    }

    pub fn cycle_active_tone(&mut self) {
        self.active_tone_index = (self.active_tone_index + 1) % self.tones.len();
    }
}
