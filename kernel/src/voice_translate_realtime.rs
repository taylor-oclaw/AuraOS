extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct VoiceTranslateRealtime {
    // Placeholder for internal state
    translations: Vec<(String, String)>,
}

impl VoiceTranslateRealtime {
    pub fn new() -> Self {
        VoiceTranslateRealtime {
            translations: Vec::new(),
        }
    }

    pub fn add_translation(&mut self, from: &str, to: &str) {
        let from_str = String::from(from);
        let to_str = String::from(to);
        self.translations.push((from_str, to_str));
    }

    pub fn get_translation(&self, from: &str) -> Option<&String> {
        for (f, t) in &self.translations {
            if f == from {
                return Some(t);
            }
        }
        None
    }

    pub fn remove_translation(&mut self, from: &str) {
        self.translations.retain(|(f, _)| f != from);
    }

    pub fn list_translations(&self) -> Vec<&String> {
        self.translations.iter().map(|(_, t)| t).collect()
    }

    pub fn clear_translations(&mut self) {
        self.translations.clear();
    }
}
