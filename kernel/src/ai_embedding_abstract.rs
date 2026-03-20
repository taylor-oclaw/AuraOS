extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    loop {}
}

struct AIEmbeddingAbstract {
    embeddings: Vec<Vec<f32>>,
    labels: Vec<String>,
}

impl AIEmbeddingAbstract {
    pub fn new() -> Self {
        AIEmbeddingAbstract {
            embeddings: Vec::new(),
            labels: Vec::new(),
        }
    }

    pub fn add_embedding(&mut self, embedding: Vec<f32>, label: String) {
        self.embeddings.push(embedding);
        self.labels.push(label);
    }

    pub fn get_embedding(&self, index: usize) -> Option<&Vec<f32>> {
        self.embeddings.get(index)
    }

    pub fn get_label(&self, index: usize) -> Option<&String> {
        self.labels.get(index)
    }

    pub fn find_by_label(&self, label: &str) -> Vec<usize> {
        let mut indices = Vec::new();
        for (i, l) in self.labels.iter().enumerate() {
            if l == label {
                indices.push(i);
            }
        }
        indices
    }

    pub fn remove_embedding(&mut self, index: usize) -> Option<(Vec<f32>, String)> {
        if index < self.embeddings.len() && index < self.labels.len() {
            Some((self.embeddings.remove(index), self.labels.remove(index)))
        } else {
            None
        }
    }
}
