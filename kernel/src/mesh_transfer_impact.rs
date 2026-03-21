extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn mesh_transfer_impact_init() {
    // Initialization logic for the module
}

pub extern "C" fn mesh_transfer_impact_exit() {
    // Cleanup logic for the module
}

pub struct MeshTransferImpact {
    data: Vec<u8>,
    metadata: String,
}

impl MeshTransferImpact {
    pub fn new(data: Vec<u8>, metadata: String) -> Self {
        MeshTransferImpact { data, metadata }
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    pub fn set_data(&mut self, data: Vec<u8>) {
        self.data = data;
    }

    pub fn append_data(&mut self, additional_data: &[u8]) {
        self.data.extend_from_slice(additional_data);
    }

    pub fn get_metadata(&self) -> &str {
        &self.metadata
    }

    pub fn set_metadata(&mut self, metadata: String) {
        self.metadata = metadata;
    }
}
