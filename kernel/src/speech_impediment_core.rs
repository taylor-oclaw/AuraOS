extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn speech_impediment_core_init() {
    // Initialization logic for the module
}

pub extern "C" fn speech_impediment_core_exit() {
    // Cleanup logic for the module
}

pub struct SpeechImpedimentCore {
    data: Vec<String>,
}

impl SpeechImpedimentCore {
    pub fn new() -> Self {
        SpeechImpedimentCore {
            data: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, entry: String) {
        self.data.push(entry);
    }

    pub fn remove_entry(&mut self, index: usize) -> Option<String> {
        if index < self.data.len() {
            Some(self.data.remove(index))
        } else {
            None
        }
    }

    pub fn get_entry(&self, index: usize) -> Option<&String> {
        self.data.get(index)
    }

    pub fn clear_entries(&mut self) {
        self.data.clear();
    }

    pub fn count_entries(&self) -> usize {
        self.data.len()
    }
}
