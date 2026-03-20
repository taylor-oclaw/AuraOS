extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct IntentParser {
    intents: Vec<String>,
}

impl IntentParser {
    pub fn new() -> Self {
        IntentParser {
            intents: Vec::new(),
        }
    }

    pub fn add_intent(&mut self, intent: &str) {
        self.intents.push(String::from(intent));
    }

    pub fn remove_intent(&mut self, intent: &str) {
        if let Some(index) = self.intents.iter().position(|i| i == intent) {
            self.intents.remove(index);
        }
    }

    pub fn get_intents(&self) -> &[String] {
        &self.intents
    }

    pub fn has_intent(&self, intent: &str) -> bool {
        self.intents.contains(&String::from(intent))
    }

    pub fn clear_intents(&mut self) {
        self.intents.clear();
    }
}
