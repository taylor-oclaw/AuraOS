extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn speech_impediment_learn_init() {
    // Initialization logic for the module
}

pub extern "C" fn speech_impediment_learn_exit() {
    // Cleanup logic for the module
}

pub struct SpeechImpedimentLearn {
    data: Vec<String>,
    errors: Vec<String>,
}

impl SpeechImpedimentLearn {
    pub fn new() -> Self {
        SpeechImpedimentLearn {
            data: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn add_data(&mut self, item: String) {
        self.data.push(item);
    }

    pub fn get_data(&self) -> &Vec<String> {
        &self.data
    }

    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
    }

    pub fn get_errors(&self) -> &Vec<String> {
        &self.errors
    }

    pub fn clear_data(&mut self) {
        self.data.clear();
    }
}
