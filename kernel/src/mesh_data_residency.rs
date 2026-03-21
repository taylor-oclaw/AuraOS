extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn mesh_data_residency_init() {
    // Initialization logic for the module
}

pub extern "C" fn mesh_data_residency_exit() {
    // Cleanup logic for the module
}

pub struct MeshDataResidency {
    data: Vec<u8>,
    metadata: String,
}

impl MeshDataResidency {
    pub fn new(data: Vec<u8>, metadata: &str) -> Self {
        MeshDataResidency {
            data,
            metadata: String::from(metadata),
        }
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    pub fn set_data(&mut self, new_data: Vec<u8>) {
        self.data = new_data;
    }

    pub fn get_metadata(&self) -> &str {
        &self.metadata
    }

    pub fn update_metadata(&mut self, new_metadata: &str) {
        self.metadata = String::from(new_metadata);
    }

    pub fn append_to_data(&mut self, additional_data: &[u8]) {
        self.data.extend_from_slice(additional_data);
    }
}
