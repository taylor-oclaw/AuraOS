extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct PredictNextAction {
    history: Vec<String>,
}

impl PredictNextAction {
    pub fn new() -> Self {
        PredictNextAction {
            history: Vec::new(),
        }
    }

    pub fn add_action(&mut self, action: String) {
        self.history.push(action);
    }

    pub fn get_last_action(&self) -> Option<&String> {
        self.history.last()
    }

    pub fn get_history_length(&self) -> usize {
        self.history.len()
    }

    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    pub fn predict_next(&self) -> String {
        if let Some(last_action) = self.get_last_action() {
            format!("next after {}", last_action)
        } else {
            "no history available".to_string()
        }
    }
}
