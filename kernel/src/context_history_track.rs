extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct ContextHistoryTrack {
    history: Vec<String>,
}

impl ContextHistoryTrack {
    pub fn new() -> Self {
        ContextHistoryTrack {
            history: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, entry: String) {
        if !entry.is_empty() {
            self.history.push(entry);
        }
    }

    pub fn get_last_entry(&self) -> Option<&String> {
        self.history.last()
    }

    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    pub fn get_history_size(&self) -> usize {
        self.history.len()
    }

    pub fn get_all_entries(&self) -> &[String] {
        &self.history
    }
}
