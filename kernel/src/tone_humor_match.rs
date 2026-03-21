extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ToneHumorMatch {
    tone_database: Vec<String>,
    humor_database: Vec<String>,
}

impl ToneHumorMatch {
    pub fn new() -> Self {
        ToneHumorMatch {
            tone_database: Vec::new(),
            humor_database: Vec::new(),
        }
    }

    pub fn add_tone(&mut self, tone: String) {
        self.tone_database.push(tone);
    }

    pub fn add_humor(&mut self, humor: String) {
        self.humor_database.push(humor);
    }

    pub fn get_tone_count(&self) -> usize {
        self.tone_database.len()
    }

    pub fn get_humor_count(&self) -> usize {
        self.humor_database.len()
    }

    pub fn match_tone(&self, input: &str) -> Option<&String> {
        for tone in &self.tone_database {
            if input.contains(tone) {
                return Some(tone);
            }
        }
        None
    }

    pub fn match_humor(&self, input: &str) -> Option<&String> {
        for humor in &self.humor_database {
            if input.contains(humor) {
                return Some(humor);
            }
        }
        None
    }
}
