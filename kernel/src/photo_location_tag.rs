extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct PhotoLocationTag {
    location: String,
    tags: Vec<String>,
}

impl PhotoLocationTag {
    pub fn new(location: &str) -> Self {
        PhotoLocationTag {
            location: String::from(location),
            tags: Vec::new(),
        }
    }

    pub fn add_tag(&mut self, tag: &str) {
        self.tags.push(String::from(tag));
    }

    pub fn remove_tag(&mut self, tag: &str) -> bool {
        if let Some(index) = self.tags.iter().position(|t| t == tag) {
            self.tags.remove(index);
            true
        } else {
            false
        }
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.contains(&String::from(tag))
    }

    pub fn get_location(&self) -> &str {
        &self.location
    }

    pub fn list_tags(&self) -> Vec<&str> {
        self.tags.iter().map(|t| t.as_str()).collect()
    }
}
