extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn mesh_laptop_desktop_link_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn mesh_laptop_desktop_link_exit() {
    // Cleanup logic for the module
}

pub struct MeshNode {
    id: String,
    connections: Vec<String>,
}

impl MeshNode {
    pub fn new(id: &str) -> Self {
        MeshNode {
            id: String::from(id),
            connections: Vec::new(),
        }
    }

    pub fn connect(&mut self, node_id: &str) {
        if !self.connections.contains(&String::from(node_id)) {
            self.connections.push(String::from(node_id));
        }
    }

    pub fn disconnect(&mut self, node_id: &str) {
        self.connections.retain(|conn| conn != node_id);
    }

    pub fn is_connected_to(&self, node_id: &str) -> bool {
        self.connections.contains(&String::from(node_id))
    }

    pub fn list_connections(&self) -> Vec<String> {
        self.connections.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh_node() {
        let mut node = MeshNode::new("node1");
        assert_eq!(node.id, "node1");
        assert_eq!(node.list_connections().len(), 0);

        node.connect("node2");
        assert!(node.is_connected_to("node2"));
        assert_eq!(node.list_connections().len(), 1);

        node.disconnect("node2");
        assert!(!node.is_connected_to("node2"));
        assert_eq!(node.list_connections().len(), 0);
    }
}
