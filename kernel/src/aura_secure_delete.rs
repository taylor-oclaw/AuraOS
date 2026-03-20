extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_ffi_init() {
    // Initialization code if needed
}

pub extern "C" fn rust_ffi_exit() {
    // Cleanup code if needed
}

pub struct AuraSecureDelete {
    data: Vec<u8>,
    is_deleted: bool,
}

impl AuraSecureDelete {
    pub fn new(data: Vec<u8>) -> Self {
        AuraSecureDelete {
            data,
            is_deleted: false,
        }
    }

    pub fn delete(&mut self) {
        if !self.is_deleted {
            for byte in &mut self.data {
                *byte = 0;
            }
            self.is_deleted = true;
        }
    }

    pub fn is_data_deleted(&self) -> bool {
        self.is_deleted
    }

    pub fn get_data_size(&self) -> usize {
        self.data.len()
    }

    pub fn reset(&mut self, new_data: Vec<u8>) {
        if self.is_deleted {
            self.data = new_data;
            self.is_deleted = false;
        }
    }
}
