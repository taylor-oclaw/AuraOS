extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct MeshBatchQueue {
    queue: Vec<String>,
}

impl MeshBatchQueue {
    pub fn new() -> Self {
        MeshBatchQueue {
            queue: Vec::new(),
        }
    }

    pub fn add_mesh(&mut self, mesh_name: String) {
        self.queue.push(mesh_name);
    }

    pub fn remove_mesh(&mut self, index: usize) -> Option<String> {
        if index < self.queue.len() {
            Some(self.queue.remove(index))
        } else {
            None
        }
    }

    pub fn get_mesh(&self, index: usize) -> Option<&String> {
        self.queue.get(index)
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }
}
