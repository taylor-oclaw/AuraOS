extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn mesh_forensic_capture_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn mesh_forensic_capture_exit() {
    // Cleanup logic for the module
}

pub struct MeshForensicCapture {
    data: Vec<u8>,
    metadata: String,
}

impl MeshForensicCapture {
    pub fn new(metadata: &str) -> Self {
        MeshForensicCapture {
            data: Vec::new(),
            metadata: String::from(metadata),
        }
    }

    pub fn add_data(&mut self, data: &[u8]) {
        self.data.extend_from_slice(data);
    }

    pub fn get_metadata(&self) -> &str {
        &self.metadata
    }

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn get_data_size(&self) -> usize {
        self.data.len()
    }

    pub fn extract_data(&self, start: usize, end: usize) -> Option<Vec<u8>> {
        if start <= end && end <= self.data.len() {
            Some(self.data[start..end].to_vec())
        } else {
            None
        }
    }
}
