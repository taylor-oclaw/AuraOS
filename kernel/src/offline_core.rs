extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct OfflineCore {
    data: Vec<u8>,
    metadata: String,
}

impl OfflineCore {
    pub fn new() -> Self {
        OfflineCore {
            data: Vec::new(),
            metadata: String::from(""),
        }
    }

    pub fn add_data(&mut self, bytes: &[u8]) {
        self.data.extend_from_slice(bytes);
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    pub fn set_metadata(&mut self, metadata: String) {
        self.metadata = metadata;
    }

    pub fn get_metadata(&self) -> &str {
        &self.metadata
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.metadata.clear();
    }
}
