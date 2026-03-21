extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct VoiceToIntent {
    intents: Vec<String>,
}

impl VoiceToIntent {
    pub fn new() -> Self {
        VoiceToIntent {
            intents: Vec::new(),
        }
    }

    pub fn add_intent(&mut self, intent: String) {
        self.intents.push(intent);
    }

    pub fn remove_intent(&mut self, intent: &str) -> bool {
        if let Some(index) = self.intents.iter().position(|i| i == intent) {
            self.intents.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_intents(&self) -> &[String] {
        &self.intents
    }

    pub fn has_intent(&self, intent: &str) -> bool {
        self.intents.contains(&intent.to_string())
    }

    pub fn clear_intents(&mut self) {
        self.intents.clear();
    }
}
