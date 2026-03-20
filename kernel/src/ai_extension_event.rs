extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn ai_extension_event_init() {
    // Initialization logic for the AI extension event module
}

#[no_mangle]
pub extern "C" fn ai_extension_event_exit() {
    // Cleanup logic for the AI extension event module
}

pub struct AiExtensionEvent {
    events: Vec<String>,
}

impl AiExtensionEvent {
    pub fn new() -> Self {
        AiExtensionEvent {
            events: Vec::new(),
        }
    }

    pub fn add_event(&mut self, event: &str) {
        self.events.push(event.to_string());
    }

    pub fn remove_event(&mut self, index: usize) -> Option<String> {
        if index < self.events.len() {
            Some(self.events.remove(index))
        } else {
            None
        }
    }

    pub fn get_event(&self, index: usize) -> Option<&String> {
        self.events.get(index)
    }

    pub fn list_events(&self) -> &Vec<String> {
        &self.events
    }

    pub fn clear_events(&mut self) {
        self.events.clear();
    }
}
