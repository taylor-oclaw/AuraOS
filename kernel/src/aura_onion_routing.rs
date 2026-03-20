extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_ffi_init() {
    // Initialization code for the module
}

pub extern "C" fn rust_ffi_exit() {
    // Cleanup code for the module
}

pub struct OnionRouter {
    nodes: Vec<String>,
    current_node_index: usize,
}

impl OnionRouter {
    pub fn new(nodes: Vec<String>) -> Self {
        OnionRouter {
            nodes,
            current_node_index: 0,
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

    pub fn get_current_node(&self) -> Option<&String> {
        if self.current_node_index < self.nodes.len() {
            Some(&self.nodes[self.current_node_index])
        } else {
            None
        }
    }

    pub fn next_node(&mut self) -> Option<&String> {
        if self.current_node_index < self.nodes.len() {
            let current = &self.nodes[self.current_node_index];
            self.current_node_index += 1;
            Some(current)
        } else {
            self.current_node_index = 0; // Reset to start
            None
        }
    }

    pub fn reset(&mut self) {
        self.current_node_index = 0;
    }
}
