extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn speech_rhotacism_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn speech_rhotacism_exit() {
    // Cleanup logic for the module
}

pub struct SpeechRhotacism {
    data: Vec<String>,
}

impl SpeechRhotacism {
    pub fn new() -> Self {
        SpeechRhotacism {
            data: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, entry: String) {
        self.data.push(entry);
    }

    pub fn get_entries(&self) -> &Vec<String> {
        &self.data
    }

    pub fn remove_entry(&mut self, index: usize) -> Option<String> {
        if index < self.data.len() {
            Some(self.data.remove(index))
        } else {
            None
        }
    }

    pub fn find_entry(&self, query: &str) -> Option<&String> {
        self.data.iter().find(|&&entry| entry.contains(query))
    }

    pub fn clear_entries(&mut self) {
        self.data.clear();
    }
}
