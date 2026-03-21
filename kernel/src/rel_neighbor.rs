extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rel_neighbor_init() {
    // Initialization logic for the module
}

pub extern "C" fn rel_neighbor_exit() {
    // Cleanup logic for the module
}

pub struct NeighborManager {
    neighbors: Vec<String>,
}

impl NeighborManager {
    pub fn new() -> Self {
        NeighborManager {
            neighbors: Vec::new(),
        }
    }

    pub fn add_neighbor(&mut self, neighbor: String) {
        if !self.neighbors.contains(&neighbor) {
            self.neighbors.push(neighbor);
        }
    }

    pub fn remove_neighbor(&mut self, neighbor: &str) -> bool {
        let position = self.neighbors.iter().position(|n| n == neighbor);
        if let Some(pos) = position {
            self.neighbors.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn get_neighbors(&self) -> &[String] {
        &self.neighbors
    }

    pub fn has_neighbor(&self, neighbor: &str) -> bool {
        self.neighbors.contains(&neighbor.to_string())
    }

    pub fn count_neighbors(&self) -> usize {
        self.neighbors.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_remove_neighbor() {
        let mut manager = NeighborManager::new();
        assert_eq!(manager.count_neighbors(), 0);

        manager.add_neighbor(String::from("192.168.1.1"));
        assert_eq!(manager.count_neighbors(), 1);
        assert!(manager.has_neighbor("192.168.1.1"));

        manager.remove_neighbor("192.168.1.1");
        assert_eq!(manager.count_neighbors(), 0);
        assert!(!manager.has_neighbor("192.168.1.1"));
    }

    #[test]
    fn test_get_neighbors() {
        let mut manager = NeighborManager::new();
        manager.add_neighbor(String::from("192.168.1.1"));
        manager.add_neighbor(String::from("192.168.1.2"));

        let neighbors = manager.get_neighbors();
        assert_eq!(neighbors.len(), 2);
        assert!(neighbors.contains(&String::from("192.168.1.1")));
        assert!(neighbors.contains(&String::from("192.168.1.2")));
    }
}
