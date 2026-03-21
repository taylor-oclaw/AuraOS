extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct LangPreferenceMgr {
    preferences: Vec<String>,
}

impl LangPreferenceMgr {
    pub fn new() -> Self {
        LangPreferenceMgr {
            preferences: Vec::new(),
        }
    }

    pub fn add_preference(&mut self, lang: &str) {
        if !self.preferences.contains(&lang.to_string()) {
            self.preferences.push(lang.to_string());
        }
    }

    pub fn remove_preference(&mut self, lang: &str) {
        self.preferences.retain(|pref| pref != lang);
    }

    pub fn get_preferences(&self) -> Vec<String> {
        self.preferences.clone()
    }

    pub fn has_preference(&self, lang: &str) -> bool {
        self.preferences.contains(&lang.to_string())
    }

    pub fn clear_preferences(&mut self) {
        self.preferences.clear();
    }
}
