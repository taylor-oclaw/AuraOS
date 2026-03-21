extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut tag = AISecSyntheticMediaTag::new();
    tag.add_tag("AI-Generated");
    tag.add_tag("Deepfake");
    tag.add_tag("Synthetic");

    if tag.has_tag("AI-Generated") {
    }

    let tags = tag.get_tags();
    for tag in tags.iter() {
    }

    let count = tag.tag_count();

    tag.remove_tag("Deepfake");
    let updated_tags = tag.get_tags();
    for tag in updated_tags.iter() {
    }
}

pub struct AISecSyntheticMediaTag {
    tags: Vec<String>,
}

impl AISecSyntheticMediaTag {
    pub fn new() -> Self {
        AISecSyntheticMediaTag { tags: Vec::new() }
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

    pub fn tag_count(&self) -> usize {
        self.tags.len()
    }
}
