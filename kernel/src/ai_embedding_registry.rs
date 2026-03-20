extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Initialize the module
    let mut registry = AIEmbeddingRegistry::new();
    
    // Example usage of the methods
    registry.register_embedding("model1", vec![0.1, 0.2, 0.3]);
    registry.register_embedding("model2", vec![0.4, 0.5, 0.6]);
    
    if let Some(embedding) = registry.get_embedding("model1") {
        // Do something with the embedding
    }
    
    let models = registry.list_models();
    for model in models.iter() {
        // Process each model name
    }
    
    registry.remove_embedding("model2");
}

pub struct AIEmbeddingRegistry {
    embeddings: Vec<(String, Vec<f32>)>,
}

impl AIEmbeddingRegistry {
    pub fn new() -> Self {
        AIEmbeddingRegistry {
            embeddings: Vec::new(),
        }
    }

    pub fn register_embedding(&mut self, name: &str, embedding: Vec<f32>) {
        let name = String::from(name);
        self.embeddings.push((name, embedding));
    }

    pub fn get_embedding(&self, name: &str) -> Option<&Vec<f32>> {
        for (model_name, embedding) in self.embeddings.iter() {
            if model_name == name {
                return Some(embedding);
            }
        }
        None
    }

    pub fn list_models(&self) -> Vec<String> {
        self.embeddings.iter().map(|(name, _)| name.clone()).collect()
    }

    pub fn remove_embedding(&mut self, name: &str) {
        self.embeddings.retain(|(model_name, _)| model_name != name);
    }
}
