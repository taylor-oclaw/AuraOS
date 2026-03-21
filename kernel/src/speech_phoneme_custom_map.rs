extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct SpeechPhonemeCustomMap {
    map: Vec<(String, String)>,
}

impl SpeechPhonemeCustomMap {
    pub fn new() -> Self {
        SpeechPhonemeCustomMap { map: Vec::new() }
    }

    pub fn add_mapping(&mut self, phoneme: &str, custom_map: &str) {
        let key = String::from(phoneme);
        let value = String::from(custom_map);
        self.map.push((key, value));
    }

    pub fn get_custom_map(&self, phoneme: &str) -> Option<&String> {
        for (p, custom_map) in &self.map {
            if p == phoneme {
                return Some(custom_map);
            }
        }
        None
    }

    pub fn remove_mapping(&mut self, phoneme: &str) {
        self.map.retain(|(p, _)| p != phoneme);
    }

    pub fn contains_phoneme(&self, phoneme: &str) -> bool {
        for (p, _) in &self.map {
            if p == phoneme {
                return true;
            }
        }
        false
    }

    pub fn get_all_mappings(&self) -> Vec<(&String, &String)> {
        self.map.iter().map(|(k, v)| (k, v)).collect()
    }
}
