extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ContextTravelingDetect {
    history: Vec<String>,
    current_context: String,
}

impl ContextTravelingDetect {
    pub fn new() -> Self {
        ContextTravelingDetect {
            history: Vec::new(),
            current_context: String::from("initial"),
        }
    }

    pub fn set_current_context(&mut self, context: &str) {
        if self.current_context != context {
            self.history.push(self.current_context.clone());
            self.current_context = String::from(context);
        }
    }

    pub fn get_current_context(&self) -> &str {
        &self.current_context
    }

    pub fn get_history(&self) -> &[String] {
        &self.history
    }

    pub fn revert_to_previous_context(&mut self) {
        if let Some(previous_context) = self.history.pop() {
            self.current_context = previous_context;
        }
    }

    pub fn clear_history(&mut self) {
        self.history.clear();
    }
}
