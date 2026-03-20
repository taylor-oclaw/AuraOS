extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct AgentIntentRouter {
    intents: Vec<String>,
}

impl AgentIntentRouter {
    pub fn new() -> Self {
        AgentIntentRouter {
            intents: Vec::new(),
        }
    }

    pub fn add_intent(&mut self, intent: String) {
        if !self.intents.contains(&intent) {
            self.intents.push(intent);
        }
    }

    pub fn remove_intent(&mut self, intent: &str) -> bool {
        let position = self.intents.iter().position(|x| x == intent);
        match position {
            Some(index) => {
                self.intents.remove(index);
                true
            }
            None => false,
        }
    }

    pub fn has_intent(&self, intent: &str) -> bool {
        self.intents.contains(&String::from(intent))
    }

    pub fn list_intents(&self) -> Vec<String> {
        self.intents.clone()
    }

    pub fn clear_intents(&mut self) {
        self.intents.clear();
    }
}
