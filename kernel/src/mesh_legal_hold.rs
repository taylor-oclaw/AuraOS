extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn mesh_legal_hold_init() -> i32 {
    0
}

pub extern "C" fn mesh_legal_hold_exit() {
}

struct MeshLegalHold {
    data: Vec<u8>,
    metadata: String,
}

impl MeshLegalHold {
    pub fn new(data: Vec<u8>, metadata: &str) -> Self {
        MeshLegalHold {
            data,
            metadata: String::from(metadata),
        }
    }

    pub fn add_data(&mut self, additional_data: &[u8]) {
        self.data.extend_from_slice(additional_data);
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    pub fn update_metadata(&mut self, new_metadata: &str) {
        self.metadata = String::from(new_metadata);
    }

    pub fn get_metadata(&self) -> &str {
        &self.metadata
    }

    pub fn clear_data(&mut self) {
        self.data.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh_legal_hold() {
        let mut mlh = MeshLegalHold::new(vec![1, 2, 3], "Initial metadata");
        assert_eq!(mlh.get_data(), &[1, 2, 3]);
        assert_eq!(mlh.get_metadata(), "Initial metadata");

        mlh.add_data(&[4, 5, 6]);
        assert_eq!(mlh.get_data(), &[1, 2, 3, 4, 5, 6]);

        mlh.update_metadata("Updated metadata");
        assert_eq!(mlh.get_metadata(), "Updated metadata");

        mlh.clear_data();
        assert_eq!(mlh.get_data(), &[]);
    }
}
