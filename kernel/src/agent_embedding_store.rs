extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct AgentEmbeddingStore {
    embeddings: Vec<(String, Vec<f32>)>,
}

impl AgentEmbeddingStore {
    pub fn new() -> Self {
        AgentEmbeddingStore {
            embeddings: Vec::new(),
        }
    }

    pub fn add_embedding(&mut self, name: String, embedding: Vec<f32>) {
        self.embeddings.push((name, embedding));
    }

    pub fn get_embedding(&self, name: &str) -> Option<&Vec<f32>> {
        for (n, e) in &self.embeddings {
            if n == name {
                return Some(e);
            }
        }
        None
    }

    pub fn remove_embedding(&mut self, name: &str) {
        self.embeddings.retain(|(n, _)| n != name);
    }

    pub fn list_embeddings(&self) -> Vec<&String> {
        self.embeddings.iter().map(|(n, _)| n).collect()
    }

    pub fn count_embeddings(&self) -> usize {
        self.embeddings.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_embedding_store() {
        let mut store = AgentEmbeddingStore::new();
        assert_eq!(store.count_embeddings(), 0);

        store.add_embedding(String::from("agent1"), vec![0.1, 0.2, 0.3]);
        assert_eq!(store.count_embeddings(), 1);
        assert!(store.get_embedding("agent1").is_some());

        let embedding = store.get_embedding("agent1").unwrap();
        assert_eq!(*embedding, vec![0.1, 0.2, 0.3]);

        store.remove_embedding("agent1");
        assert_eq!(store.count_embeddings(), 0);
        assert!(store.get_embedding("agent1").is_none());

        let names = store.list_embeddings();
        assert!(names.is_empty());
    }
}
