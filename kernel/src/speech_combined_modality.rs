extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn speech_combined_modality_init() {
    // Initialization logic for the module
}

pub extern "C" fn speech_combined_modality_exit() {
    // Cleanup logic for the module
}

pub struct SpeechCombinedModality {
    data: Vec<String>,
}

impl SpeechCombinedModality {
    pub fn new() -> Self {
        SpeechCombinedModality {
            data: Vec::new(),
        }
    }

    pub fn add_data(&mut self, item: String) {
        self.data.push(item);
    }

    pub fn get_data(&self) -> &Vec<String> {
        &self.data
    }

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn find_data(&self, query: &str) -> Option<&String> {
        self.data.iter().find(|&&item| item.contains(query))
    }

    pub fn remove_data(&mut self, index: usize) -> Option<String> {
        if index < self.data.len() {
            Some(self.data.remove(index))
        } else {
            None
        }
    }
}
