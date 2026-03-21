extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct MeshDataLocality {
    vertices: Vec<[f32; 3]>,
    indices: Vec<u32>,
}

impl MeshDataLocality {
    pub fn new() -> Self {
        MeshDataLocality {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    pub fn add_vertex(&mut self, vertex: [f32; 3]) {
        self.vertices.push(vertex);
    }

    pub fn add_triangle(&mut self, index1: u32, index2: u32, index3: u32) {
        if index1 < self.vertices.len() as u32
            && index2 < self.vertices.len() as u32
            && index3 < self.vertices.len() as u32
        {
            self.indices.push(index1);
            self.indices.push(index2);
            self.indices.push(index3);
        }
    }

    pub fn get_vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn get_triangle_count(&self) -> usize {
        self.indices.len() / 3
    }

    pub fn get_vertices(&self) -> &[f32; 3] {
        &self.vertices
    }

    pub fn get_indices(&self) -> &[u32] {
        &self.indices
    }
}
