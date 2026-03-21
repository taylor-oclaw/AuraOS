extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct MeshDataTagging {
    tags: Vec<String>,
}

impl MeshDataTagging {
    pub fn new() -> Self {
        MeshDataTagging {
            tags: Vec::new(),
        }
    }

    pub fn add_tag(&mut self, tag: &str) {
        if !self.tags.contains(&String::from(tag)) {
            self.tags.push(String::from(tag));
        }
    }

    pub fn remove_tag(&mut self, tag: &str) {
        self.tags.retain(|t| t != tag);
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.contains(&String::from(tag))
    }

    pub fn get_tags(&self) -> Vec<String> {
        self.tags.clone()
    }

    pub fn clear_tags(&mut self) {
        self.tags.clear();
    }
}
