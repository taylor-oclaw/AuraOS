extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_ffi_init() {
    // Initialize the module
}

pub extern "C" fn rust_ffi_exit() {
    // Clean up the module
}

pub struct AuraTorBridge {
    nodes: Vec<String>,
    connections: usize,
    active: bool,
}

impl AuraTorBridge {
    pub fn new() -> Self {
        AuraTorBridge {
            nodes: Vec::new(),
            connections: 0,
            active: false,
        }
    }

    pub fn add_node(&mut self, node: String) {
        self.nodes.push(node);
    }

    pub fn remove_node(&mut self, index: usize) -> Option<String> {
        if index < self.nodes.len() {
            Some(self.nodes.remove(index))
        } else {
            None
        }
    }

    pub fn list_nodes(&self) -> Vec<String> {
        self.nodes.clone()
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn connect(&mut self) {
        if !self.nodes.is_empty() {
            self.connections += 1;
        }
    }

    pub fn disconnect(&mut self) {
        if self.connections > 0 {
            self.connections -= 1;
        }
    }

    pub fn get_connection_count(&self) -> usize {
        self.connections
    }
}
