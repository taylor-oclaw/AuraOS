extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct PhotoFaceCluster {
    faces: Vec<String>,
}

impl PhotoFaceCluster {
    pub fn new() -> Self {
        PhotoFaceCluster { faces: Vec::new() }
    }

    pub fn add_face(&mut self, face_id: String) {
        if !self.faces.contains(&face_id) {
            self.faces.push(face_id);
        }
    }

    pub fn remove_face(&mut self, face_id: &str) -> bool {
        let index = self.faces.iter().position(|f| f == face_id);
        match index {
            Some(i) => {
                self.faces.remove(i);
                true
            }
            None => false,
        }
    }

    pub fn contains_face(&self, face_id: &str) -> bool {
        self.faces.contains(&String::from(face_id))
    }

    pub fn get_faces_count(&self) -> usize {
        self.faces.len()
    }

    pub fn list_faces(&self) -> Vec<String> {
        self.faces.clone()
    }
}
