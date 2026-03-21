extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn lang_core_init() {
    // Initialization logic for the module
}

pub extern "C" fn lang_core_exit() {
    // Cleanup logic for the module
}

pub struct LangCore {
    data: Vec<u8>,
    name: String,
}

impl LangCore {
    pub fn new(name: &str) -> Self {
        LangCore {
            data: Vec::new(),
            name: String::from(name),
        }
    }

    pub fn add_data(&mut self, byte: u8) {
        self.data.push(byte);
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn set_name(&mut self, new_name: &str) {
        self.name = String::from(new_name);
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}
