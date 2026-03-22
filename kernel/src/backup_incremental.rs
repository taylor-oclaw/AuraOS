extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_backup_incremental_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn rust_backup_incremental_exit() {
    // Cleanup logic for the module
}

pub struct BackupIncremental {
    data: Vec<u8>,
    metadata: String,
}

impl BackupIncremental {
    pub fn new(data: Vec<u8>, metadata: String) -> Self {
        BackupIncremental { data, metadata }
    }

    pub fn get_data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn set_data(&mut self, data: Vec<u8>) {
        self.data = data;
    }

    pub fn get_metadata(&self) -> &String {
        &self.metadata
    }

    pub fn update_metadata(&mut self, metadata: String) {
        self.metadata = metadata;
    }

    pub fn append_data(&mut self, additional_data: Vec<u8>) {
        self.data.extend(additional_data);
    }
}