extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct AuraAssistantPersonality {
    name: String,
    traits: Vec<String>,
}

impl AuraAssistantPersonality {
    pub fn new(name: &str) -> Self {
        AuraAssistantPersonality {
            name: String::from(name),
            traits: Vec::new(),
        }
    }

    pub fn add_trait(&mut self, trait_name: &str) {
        self.traits.push(String::from(trait_name));
    }

    pub fn remove_trait(&mut self, trait_name: &str) {
        if let Some(index) = self.traits.iter().position(|t| t == trait_name) {
            self.traits.remove(index);
        }
    }

    pub fn has_trait(&self, trait_name: &str) -> bool {
        self.traits.contains(&String::from(trait_name))
    }

    pub fn list_traits(&self) -> Vec<String> {
        self.traits.clone()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}
