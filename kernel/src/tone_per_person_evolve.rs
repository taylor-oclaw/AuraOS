extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct TonePerPersonEvolve {
    person_name: String,
    tone_history: Vec<String>,
}

impl TonePerPersonEvolve {
    pub fn new(person_name: &str) -> Self {
        TonePerPersonEvolve {
            person_name: String::from(person_name),
            tone_history: Vec::new(),
        }
    }

    pub fn add_tone(&mut self, tone: &str) {
        self.tone_history.push(String::from(tone));
    }

    pub fn get_person_name(&self) -> &str {
        &self.person_name
    }

    pub fn get_last_tone(&self) -> Option<&str> {
        self.tone_history.last().map(|s| s.as_str())
    }

    pub fn get_all_tones(&self) -> Vec<&str> {
        self.tone_history.iter().map(|s| s.as_str()).collect()
    }

    pub fn clear_tone_history(&mut self) {
        self.tone_history.clear();
    }
}
