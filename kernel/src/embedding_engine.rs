extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn embedding_engine_init() {
    // Initialization logic for the embedding engine module
}

#[no_mangle]
pub extern "C" fn embedding_engine_exit() {
    // Cleanup logic for the embedding engine module
}

pub struct EmbeddingEngine {
    data: Vec<u8>,
    metadata: String,
}

impl EmbeddingEngine {
    pub fn new(data: Vec<u8>, metadata: &str) -> Self {
        EmbeddingEngine {
            data,
            metadata: String::from(metadata),
        }
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    pub fn set_data(&mut self, new_data: Vec<u8>) {
        self.data = new_data;
    }

    pub fn get_metadata(&self) -> &str {
        &self.metadata
    }

    pub fn update_metadata(&mut self, new_metadata: &str) {
        self.metadata = String::from(new_metadata);
    }

    pub fn append_to_data(&mut self, additional_data: &[u8]) {
        self.data.extend_from_slice(additional_data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embedding_engine() {
        let mut engine = EmbeddingEngine::new(vec![1, 2, 3], "initial metadata");

        assert_eq!(engine.get_data(), &[1, 2, 3]);
        assert_eq!(engine.get_metadata(), "initial metadata");

        engine.set_data(vec![4, 5, 6]);
        assert_eq!(engine.get_data(), &[4, 5, 6]);

        engine.update_metadata("updated metadata");
        assert_eq!(engine.get_metadata(), "updated metadata");

        engine.append_to_data(&[7, 8, 9]);
        assert_eq!(engine.get_data(), &[4, 5, 6, 7, 8, 9]);
    }
}
