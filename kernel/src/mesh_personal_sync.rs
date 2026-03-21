extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn mesh_personal_sync_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn mesh_personal_sync_exit() {
    // Cleanup logic for the module
}

pub struct MeshPersonalSync {
    nodes: Vec<String>,
    data_store: Vec<(String, String)>, // (key, value)
}

impl MeshPersonalSync {
    pub fn new() -> Self {
        MeshPersonalSync {
            nodes: Vec::new(),
            data_store: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node_id: &str) {
        if !self.nodes.contains(&node_id.to_string()) {
            self.nodes.push(node_id.to_string());
        }
    }

    pub fn remove_node(&mut self, node_id: &str) {
        self.nodes.retain(|n| n != node_id);
    }

    pub fn get_nodes(&self) -> Vec<String> {
        self.nodes.clone()
    }

    pub fn store_data(&mut self, key: &str, value: &str) {
        if let Some(index) = self.data_store.iter().position(|(k, _)| k == key) {
            self.data_store[index] = (key.to_string(), value.to_string());
        } else {
            self.data_store.push((key.to_string(), value.to_string()));
        }
    }

    pub fn retrieve_data(&self, key: &str) -> Option<String> {
        self.data_store.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone())
    }

    pub fn remove_data(&mut self, key: &str) {
        self.data_store.retain(|(k, _)| k != key);
    }
}
