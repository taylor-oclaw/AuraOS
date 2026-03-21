extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct BizHandoffRelationship {
    relationships: Vec<(String, String)>,
}

impl BizHandoffRelationship {
    pub fn new() -> Self {
        BizHandoffRelationship {
            relationships: Vec::new(),
        }
    }

    pub fn add_relationship(&mut self, from: &str, to: &str) {
        self.relationships.push((from.to_string(), to.to_string()));
    }

    pub fn remove_relationship(&mut self, from: &str, to: &str) -> bool {
        let pos = self
            .relationships
            .iter()
            .position(|&(ref f, ref t)| f == from && t == to);
        if let Some(index) = pos {
            self.relationships.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_relationships(&self) -> &Vec<(String, String)> {
        &self.relationships
    }

    pub fn has_relationship(&self, from: &str, to: &str) -> bool {
        self.relationships.iter().any(|&(ref f, ref t)| f == from && t == to)
    }

    pub fn count_relationships_from(&self, from: &str) -> usize {
        self.relationships.iter().filter(|&&(ref f, _)| f == from).count()
    }
}
