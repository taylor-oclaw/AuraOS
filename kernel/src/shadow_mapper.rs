extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ShadowMapper {
    mappings: Vec<(String, String)>,
}

impl ShadowMapper {
    pub fn new() -> Self {
        ShadowMapper {
            mappings: Vec::new(),
        }
    }

    pub fn add_mapping(&mut self, original: &str, shadow: &str) {
        let original = String::from(original);
        let shadow = String::from(shadow);
        self.mappings.push((original, shadow));
    }

    pub fn get_shadow(&self, original: &str) -> Option<&String> {
        for (o, s) in &self.mappings {
            if o == original {
                return Some(s);
            }
        }
        None
    }

    pub fn remove_mapping(&mut self, original: &str) -> bool {
        let pos = self.mappings.iter().position(|(o, _)| o == original);
        if let Some(index) = pos {
            self.mappings.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_mappings(&self) -> Vec<(String, String)> {
        self.mappings.clone()
    }

    pub fn clear_mappings(&mut self) {
        self.mappings.clear();
    }
}
