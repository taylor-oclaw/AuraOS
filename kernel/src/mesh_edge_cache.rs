extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MeshEdgeCache {
    edges: Vec<Edge>,
}

impl MeshEdgeCache {
    pub fn new() -> Self {
        MeshEdgeCache { edges: Vec::new() }
    }

    pub fn add_edge(&mut self, edge: Edge) {
        if !self.edges.contains(&edge) {
            self.edges.push(edge);
        }
    }

    pub fn remove_edge(&mut self, edge: &Edge) -> bool {
        let index = self.edges.iter().position(|e| e == edge);
        if let Some(i) = index {
            self.edges.remove(i);
            true
        } else {
            false
        }
    }

    pub fn contains_edge(&self, edge: &Edge) -> bool {
        self.edges.contains(edge)
    }

    pub fn get_edges_count(&self) -> usize {
        self.edges.len()
    }

    pub fn clear(&mut self) {
        self.edges.clear();
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Edge {
    vertex1: u32,
    vertex2: u32,
}

impl Edge {
    pub fn new(vertex1: u32, vertex2: u32) -> Self {
        Edge { vertex1, vertex2 }
    }

    pub fn get_vertices(&self) -> (u32, u32) {
        (self.vertex1, self.vertex2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh_edge_cache() {
        let mut cache = MeshEdgeCache::new();
        let edge1 = Edge::new(1, 2);
        let edge2 = Edge::new(3, 4);

        assert_eq!(cache.get_edges_count(), 0);
        assert!(!cache.contains_edge(&edge1));

        cache.add_edge(edge1.clone());
        assert_eq!(cache.get_edges_count(), 1);
        assert!(cache.contains_edge(&edge1));
        assert!(!cache.contains_edge(&edge2));

        cache.add_edge(edge2.clone());
        assert_eq!(cache.get_edges_count(), 2);
        assert!(cache.contains_edge(&edge2));

        assert!(cache.remove_edge(&edge1));
        assert_eq!(cache.get_edges_count(), 1);
        assert!(!cache.contains_edge(&edge1));

        cache.clear();
        assert_eq!(cache.get_edges_count(), 0);
    }
}
