extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_ffi_init() {
    // Initialization code if needed
}

#[no_mangle]
pub extern "C" fn rust_ffi_exit() {
    // Cleanup code if needed
}

pub struct AuraAssistantEveningRecap {
    events: Vec<String>,
    summary: String,
}

impl AuraAssistantEveningRecap {
    pub fn new() -> Self {
        AuraAssistantEveningRecap {
            events: Vec::new(),
            summary: String::from("No events recorded."),
        }
    }

    pub fn add_event(&mut self, event: &str) {
        self.events.push(String::from(event));
        self.update_summary();
    }

    pub fn remove_event(&mut self, index: usize) -> Option<String> {
        if let Some(removed_event) = self.events.remove(index) {
            self.update_summary();
            Some(removed_event)
        } else {
            None
        }
    }

    pub fn get_events(&self) -> &Vec<String> {
        &self.events
    }

    pub fn get_summary(&self) -> &str {
        &self.summary
    }

    fn update_summary(&mut self) {
        if self.events.is_empty() {
            self.summary = String::from("No events recorded.");
        } else {
            let mut summary = String::from("Evening Recap:\n");
            for (index, event) in self.events.iter().enumerate() {
                summary.push_str(&format!("{}. {}\n", index + 1, event));
            }
            self.summary = summary;
        }
    }
}
