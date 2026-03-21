extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct MeetingTranslateLive {
    translations: Vec<(String, String)>,
}

impl MeetingTranslateLive {
    pub fn new() -> Self {
        MeetingTranslateLive {
            translations: Vec::new(),
        }
    }

    pub fn add_translation(&mut self, source: &str, target: &str) {
        let source_str = String::from(source);
        let target_str = String::from(target);
        self.translations.push((source_str, target_str));
    }

    pub fn get_translation(&self, source: &str) -> Option<&String> {
        for (src, tgt) in &self.translations {
            if src == source {
                return Some(tgt);
            }
        }
        None
    }

    pub fn remove_translation(&mut self, source: &str) {
        self.translations.retain(|(src, _)| src != source);
    }

    pub fn list_translations(&self) -> Vec<&String> {
        self.translations.iter().map(|(_, tgt)| tgt).collect()
    }

    pub fn clear_translations(&mut self) {
        self.translations.clear();
    }
}
