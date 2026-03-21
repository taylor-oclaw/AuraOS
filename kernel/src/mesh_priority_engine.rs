extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn mesh_priority_engine_init() {
    // Initialization logic for the module
}

pub extern "C" fn mesh_priority_engine_exit() {
    // Cleanup logic for the module
}

pub struct MeshPriorityEngine {
    nodes: Vec<String>,
    priorities: Vec<u32>,
}

impl MeshPriorityEngine {
    pub fn new() -> Self {
        MeshPriorityEngine {
            nodes: Vec::new(),
            priorities: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node_name: &str, priority: u32) {
        self.nodes.push(String::from(node_name));
        self.priorities.push(priority);
    }

    pub fn get_priority(&self, node_name: &str) -> Option<u32> {
        if let Some(index) = self.nodes.iter().position(|n| n == node_name) {
            Some(self.priorities[index])
        } else {
            None
        }
    }

    pub fn update_priority(&mut self, node_name: &str, new_priority: u32) -> bool {
        if let Some(index) = self.nodes.iter().position(|n| n == node_name) {
            self.priorities[index] = new_priority;
            true
        } else {
            false
        }
    }

    pub fn remove_node(&mut self, node_name: &str) -> bool {
        if let Some(index) = self.nodes.iter().position(|n| n == node_name) {
            self.nodes.remove(index);
            self.priorities.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_all_nodes(&self) -> Vec<&String> {
        self.nodes.iter().collect()
    }
}
