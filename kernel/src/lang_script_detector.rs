extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub struct LangScriptDetector {
    // Example data structure to hold language script detection logic
    scripts: Vec<String>,
}

impl LangScriptDetector {
    pub fn new() -> Self {
        LangScriptDetector {
            scripts: vec![
                String::from("Latin"),
                String::from("Cyrillic"),
                String::from("Arabic"),
                String::from("Devanagari"),
                String::from("Chinese"),
            ],
        }
    }

    pub fn detect_script(&self, text: &str) -> Option<&String> {
        // Simple detection logic based on character ranges
        if text.chars().any(|c| matches!(c, 'a'..='z' | 'A'..='Z')) {
            self.scripts.iter().find(|s| s == &&String::from("Latin"))
        } else if text.chars().any(|c| matches!(c, '\u{0400}'..='\u{04FF}')) {
            self.scripts.iter().find(|s| s == &&String::from("Cyrillic"))
        } else if text.chars().any(|c| matches!(c, '\u{0600}'..='\u{06FF}')) {
            self.scripts.iter().find(|s| s == &&String::from("Arabic"))
        } else if text.chars().any(|c| matches!(c, '\u{0900}'..='\u{097F}')) {
            self.scripts.iter().find(|s| s == &&String::from("Devanagari"))
        } else if text.chars().any(|c| matches!(c, '\u{4E00}'..='\u{9FFF}')) {
            self.scripts.iter().find(|s| s == &&String::from("Chinese"))
        } else {
            None
        }
    }

    pub fn add_script(&mut self, script: String) {
        if !self.scripts.contains(&script) {
            self.scripts.push(script);
        }
    }

    pub fn remove_script(&mut self, script: &str) {
        if let Some(index) = self.scripts.iter().position(|s| s == script) {
            self.scripts.remove(index);
        }
    }

    pub fn list_scripts(&self) -> Vec<&String> {
        self.scripts.iter().collect()
    }

    pub fn is_script_supported(&self, script: &str) -> bool {
        self.scripts.contains(&String::from(script))
    }
}
