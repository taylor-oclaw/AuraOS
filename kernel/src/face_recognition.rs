extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct FaceRecognition {
    database: Vec<(String, [u8; 128])>, // Assuming a fixed-size vector for simplicity
}

impl FaceRecognition {
    pub fn new() -> Self {
        FaceRecognition {
            database: Vec::new(),
        }
    }

    pub fn add_face(&mut self, name: String, face_vector: [u8; 128]) {
        self.database.push((name, face_vector));
    }

    pub fn remove_face(&mut self, name: &str) -> bool {
        let pos = self.database.iter().position(|(n, _)| n == name);
        if let Some(index) = pos {
            self.database.remove(index);
            true
        } else {
            false
        }
    }

    pub fn recognize_face(&self, face_vector: &[u8; 128]) -> Option<&String> {
        for (name, stored_vector) in &self.database {
            if Self::compare_vectors(face_vector, stored_vector) < 0.1 { // Threshold for similarity
                return Some(name);
            }
        }
        None
    }

    pub fn list_faces(&self) -> Vec<String> {
        self.database.iter().map(|(name, _)| name.clone()).collect()
    }

    fn compare_vectors(v1: &[u8; 128], v2: &[u8; 128]) -> f32 {
        let mut sum = 0.0;
        for (a, b) in v1.iter().zip(v2.iter()) {
            sum += (*a as f32 - *b as f32).powi(2);
        }
        (sum / 128.0).sqrt()
    }
}
