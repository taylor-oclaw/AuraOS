extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct AuraCalculator {
    results: Vec<(String, i32)>,
}

impl AuraCalculator {
    pub fn new() -> Self {
        AuraCalculator {
            results: Vec::new(),
        }
    }

    pub fn add(&mut self, name: &str, aura_value: i32) {
        let entry = (String::from(name), aura_value);
        self.results.push(entry);
    }

    pub fn get_aura(&self, name: &str) -> Option<i32> {
        for (entry_name, value) in &self.results {
            if entry_name == name {
                return Some(*value);
            }
        }
        None
    }

    pub fn total_aura(&self) -> i32 {
        self.results.iter().map(|(_, value)| *value).sum()
    }

    pub fn average_aura(&self) -> Option<f32> {
        if self.results.is_empty() {
            return None;
        }
        let total = self.total_aura();
        Some(total as f32 / self.results.len() as f32)
    }

    pub fn highest_aura(&self) -> Option<&(String, i32)> {
        self.results.iter().max_by_key(|&(_, value)| value)
    }
}
