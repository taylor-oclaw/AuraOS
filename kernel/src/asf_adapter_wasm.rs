extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn asf_adapter_wasm_init() {
    // Initialization logic for the module
}

pub extern "C" fn asf_adapter_wasm_exit() {
    // Cleanup logic for the module
}

pub struct AsfAdapterWasm {
    data: Vec<u8>,
    name: String,
}

impl AsfAdapterWasm {
    pub fn new(name: &str) -> Self {
        AsfAdapterWasm {
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
