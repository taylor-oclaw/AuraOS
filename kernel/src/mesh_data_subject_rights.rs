extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MeshDataSubjectRights {
    subject_id: String,
    rights: Vec<String>,
}

impl MeshDataSubjectRights {
    pub fn new(subject_id: &str) -> Self {
        MeshDataSubjectRights {
            subject_id: String::from(subject_id),
            rights: Vec::new(),
        }
    }

    pub fn add_right(&mut self, right: &str) {
        if !self.rights.contains(&String::from(right)) {
            self.rights.push(String::from(right));
        }
    }

    pub fn remove_right(&mut self, right: &str) {
        self.rights.retain(|r| r != right);
    }

    pub fn has_right(&self, right: &str) -> bool {
        self.rights.contains(&String::from(right))
    }

    pub fn list_rights(&self) -> Vec<String> {
        self.rights.clone()
    }

    pub fn clear_rights(&mut self) {
        self.rights.clear();
    }
}
