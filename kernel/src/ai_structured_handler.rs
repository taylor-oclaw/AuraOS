extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn ai_structured_handler_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn ai_structured_handler_exit() {
    // Cleanup logic for the module
}

pub struct AIHandler {
    data: Vec<u8>,
    metadata: String,
}

impl AIHandler {
    pub fn new(data: Vec<u8>, metadata: &str) -> Self {
        AIHandler {
            data,
            metadata: String::from(metadata),
        }
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    pub fn set_data(&mut self, data: Vec<u8>) {
        self.data = data;
    }

    pub fn get_metadata(&self) -> &str {
        &self.metadata
    }

    pub fn update_metadata(&mut self, new_metadata: &str) {
        self.metadata = String::from(new_metadata);
    }

    pub fn append_data(&mut self, additional_data: &[u8]) {
        self.data.extend_from_slice(additional_data);
    }
}