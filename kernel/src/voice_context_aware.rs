extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct VoiceContext {
    context: String,
    history: Vec<String>,
}

impl VoiceContext {
    pub fn new() -> Self {
        VoiceContext {
            context: String::new(),
            history: Vec::new(),
        }
    }

    pub fn add_to_context(&mut self, text: &str) {
        self.context.push_str(text);
        self.history.push(String::from(text));
    }

    pub fn clear_context(&mut self) {
        self.context.clear();
        self.history.clear();
    }

    pub fn get_current_context(&self) -> &str {
        &self.context
    }

    pub fn get_history(&self) -> &[String] {
        &self.history
    }

    pub fn undo_last_entry(&mut self) {
        if let Some(last_entry) = self.history.pop() {
            self.context.retain(|c| !last_entry.contains(c));
        }
    }
}
