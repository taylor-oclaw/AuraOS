extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct PredictFocusWindow {
    history: Vec<String>,
    current_focus: String,
}

impl PredictFocusWindow {
    pub fn new(initial_focus: &str) -> Self {
        PredictFocusWindow {
            history: Vec::new(),
            current_focus: initial_focus.to_string(),
        }
    }

    pub fn add_to_history(&mut self, window_name: &str) {
        self.history.push(window_name.to_string());
    }

    pub fn get_current_focus(&self) -> &String {
        &self.current_focus
    }

    pub fn set_current_focus(&mut self, new_focus: &str) {
        self.add_to_history(&self.current_focus);
        self.current_focus = new_focus.to_string();
    }

    pub fn predict_next_focus(&self) -> Option<&String> {
        if self.history.is_empty() {
            None
        } else {
            Some(self.history.last().unwrap())
        }
    }

    pub fn clear_history(&mut self) {
        self.history.clear();
    }
}
