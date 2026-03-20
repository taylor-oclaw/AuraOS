extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn crucible_io_init() {
    // Initialization logic for the module
}

pub extern "C" fn crucible_io_exit() {
    // Cleanup logic for the module
}

pub struct CrucibleIO {
    data: Vec<u8>,
    name: String,
}

impl CrucibleIO {
    pub fn new(name: &str) -> Self {
        CrucibleIO {
            data: Vec::new(),
            name: String::from(name),
        }
    }

    pub fn read(&self, offset: usize, length: usize) -> Option<&[u8]> {
        if offset + length > self.data.len() {
            None
        } else {
            Some(&self.data[offset..offset + length])
        }
    }

    pub fn write(&mut self, data: &[u8]) {
        self.data.extend_from_slice(data);
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}
