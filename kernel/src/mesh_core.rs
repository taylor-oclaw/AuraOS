extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

mod mesh_core {
    use super::*;

    pub struct MeshNode {
        id: u32,
        name: String,
        neighbors: Vec<u32>,
    }

    impl MeshNode {
        pub fn new(id: u32, name: &str) -> Self {
            MeshNode {
                id,
                name: String::from(name),
                neighbors: Vec::new(),
            }
        }

        pub fn add_neighbor(&mut self, neighbor_id: u32) {
            if !self.neighbors.contains(&neighbor_id) {
                self.neighbors.push(neighbor_id);
            }
        }

        pub fn remove_neighbor(&mut self, neighbor_id: u32) {
            self.neighbors.retain(|&id| id != neighbor_id);
        }

        pub fn get_neighbors(&self) -> &Vec<u32> {
            &self.neighbors
        }

        pub fn has_neighbor(&self, neighbor_id: u32) -> bool {
            self.neighbors.contains(&neighbor_id)
        }

        pub fn get_name(&self) -> &str {
            &self.name
        }
    }
}

#[cfg(test)]
mod tests {
    use super::mesh_core::*;

    #[test]
    fn test_mesh_node() {
        let mut node = MeshNode::new(1, "Node1");
        assert_eq!(node.get_name(), "Node1");
        assert!(!node.has_neighbor(2));

        node.add_neighbor(2);
        assert!(node.has_neighbor(2));
        assert_eq!(node.get_neighbors().len(), 1);

        node.remove_neighbor(2);
        assert!(!node.has_neighbor(2));
        assert_eq!(node.get_neighbors().len(), 0);
    }
}
