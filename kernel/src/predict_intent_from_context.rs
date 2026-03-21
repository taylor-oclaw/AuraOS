extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct PredictIntentFromContext {
    context: Vec<String>,
    intents: Vec<String>,
}

impl PredictIntentFromContext {
    pub fn new() -> Self {
        PredictIntentFromContext {
            context: Vec::new(),
            intents: Vec::new(),
        }
    }

    pub fn add_context(&mut self, context: String) {
        self.context.push(context);
    }

    pub fn add_intent(&mut self, intent: String) {
        self.intents.push(intent);
    }

    pub fn get_context(&self) -> &Vec<String> {
        &self.context
    }

    pub fn get_intents(&self) -> &Vec<String> {
        &self.intents
    }

    pub fn predict_intent(&self, input: &str) -> Option<&String> {
        for context in &self.context {
            if input.contains(context) {
                return self.intents.first();
            }
        }
        None
    }
}
