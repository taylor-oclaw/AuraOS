extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct VoiceIntentConfirm {
    intents: Vec<String>,
    confirmed_intent: Option<String>,
}

impl VoiceIntentConfirm {
    pub fn new() -> Self {
        VoiceIntentConfirm {
            intents: Vec::new(),
            confirmed_intent: None,
        }
    }

    pub fn add_intent(&mut self, intent: String) {
        self.intents.push(intent);
    }

    pub fn get_intents(&self) -> &Vec<String> {
        &self.intents
    }

    pub fn confirm_intent(&mut self, intent: &str) -> bool {
        if let Some(index) = self.intents.iter().position(|i| i == intent) {
            self.confirmed_intent = Some(self.intents.remove(index));
            true
        } else {
            false
        }
    }

    pub fn get_confirmed_intent(&self) -> Option<&String> {
        self.confirmed_intent.as_ref()
    }

    pub fn clear_intents(&mut self) {
        self.intents.clear();
        self.confirmed_intent = None;
    }
}
