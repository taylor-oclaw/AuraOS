extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct LanguageTranslator {
    formal_to_informal: Vec<(String, String)>,
    informal_to_formal: Vec<(String, String)>,
}

impl LanguageTranslator {
    pub fn new() -> Self {
        LanguageTranslator {
            formal_to_informal: Vec::new(),
            informal_to_formal: Vec::new(),
        }
    }

    pub fn add_translation(&mut self, formal: &str, informal: &str) {
        self.formal_to_informal.push((String::from(formal), String::from(informal)));
        self.informal_to_formal.push((String::from(informal), String::from(formal)));
    }

    pub fn translate_formal_to_informal(&self, text: &str) -> String {
        let mut result = String::new();
        for (formal, informal) in &self.formal_to_informal {
            if text.contains(formal) {
                result.push_str(informal);
            } else {
                result.push_str(text);
            }
        }
        result
    }

    pub fn translate_informal_to_formal(&self, text: &str) -> String {
        let mut result = String::new();
        for (informal, formal) in &self.informal_to_formal {
            if text.contains(informal) {
                result.push_str(formal);
            } else {
                result.push_str(text);
            }
        }
        result
    }

    pub fn list_translations(&self) -> Vec<(String, String)> {
        self.formal_to_informal.clone()
    }

    pub fn clear_translations(&mut self) {
        self.formal_to_informal.clear();
        self.informal_to_formal.clear();
    }
}
