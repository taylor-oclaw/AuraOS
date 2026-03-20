extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn triton_compat_init() -> i32 {
    0
}

pub extern "C" fn triton_compat_exit() -> i32 {
    0
}

pub struct TritonCompat {
    data: Vec<u8>,
    name: String,
}

impl TritonCompat {
    pub fn new(name: &str) -> Self {
        TritonCompat {
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

    pub fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}
