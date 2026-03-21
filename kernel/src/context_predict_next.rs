extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ContextPredictNext {
    history: Vec<String>,
    predictions: Vec<String>,
}

impl ContextPredictNext {
    pub fn new() -> Self {
        ContextPredictNext {
            history: Vec::new(),
            predictions: Vec::new(),
        }
    }

    pub fn add_to_history(&mut self, context: String) {
        self.history.push(context);
    }

    pub fn get_history(&self) -> &Vec<String> {
        &self.history
    }

    pub fn generate_predictions(&mut self) {
        // Simple prediction logic based on history length
        if self.history.len() > 0 {
            let last_context = self.history.last().unwrap();
            for i in 1..=5 {
                self.predictions.push(String::from("info"));
            }
        }
    }

    pub fn get_predictions(&self) -> &Vec<String> {
        &self.predictions
    }

    pub fn clear_predictions(&mut self) {
        self.predictions.clear();
    }
}
