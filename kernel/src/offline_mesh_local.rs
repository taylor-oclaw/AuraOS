extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn offline_mesh_local_init() {
    // Initialization logic for the module
}

pub extern "C" fn offline_mesh_local_exit() {
    // Cleanup logic for the module
}

pub struct OfflineMeshLocal {
    nodes: Vec<String>,
    messages: Vec<String>,
}

impl OfflineMeshLocal {
    pub fn new() -> Self {
        OfflineMeshLocal {
            nodes: Vec::new(),
            messages: Vec::new(),
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

    pub fn broadcast_message(&mut self, message: &str) {
        self.messages.push(message.to_string());
    }

    pub fn get_messages(&self) -> Vec<String> {
        self.messages.clone()
    }

    pub fn get_node_count(&self) -> usize {
        self.nodes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offline_mesh_local() {
        let mut mesh = OfflineMeshLocal::new();
        assert_eq!(mesh.get_node_count(), 0);

        mesh.add_node("node1");
        mesh.add_node("node2");
        assert_eq!(mesh.get_node_count(), 2);

        mesh.remove_node("node1");
        assert_eq!(mesh.get_node_count(), 1);

        mesh.broadcast_message("Hello, Mesh!");
        let messages = mesh.get_messages();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0], "Hello, Mesh!");
    }
}
