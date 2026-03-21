extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct ToneDirectnessLevel {
    level: u8,
    description: String,
}

impl ToneDirectnessLevel {
    pub fn new(level: u8) -> Self {
        let description = match level {
            0 => "Very Indirect".into(),
            1 => "Indirect".into(),
            2 => "Neutral".into(),
            3 => "Direct".into(),
            4 => "Very Direct".into(),
            _ => "Invalid Level".into(),
        };
        ToneDirectnessLevel { level, description }
    }

    pub fn get_level(&self) -> u8 {
        self.level
    }

    pub fn set_level(&mut self, level: u8) {
        self.level = level;
        self.description = match level {
            0 => "Very Indirect".into(),
            1 => "Indirect".into(),
            2 => "Neutral".into(),
            3 => "Direct".into(),
            4 => "Very Direct".into(),
            _ => "Invalid Level".into(),
        };
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn is_direct(&self) -> bool {
        self.level >= 3
    }

    pub fn levels_to_string() -> Vec<String> {
        vec![
            "Very Indirect".into(),
            "Indirect".into(),
            "Neutral".into(),
            "Direct".into(),
            "Very Direct".into(),
        ]
    }
}
