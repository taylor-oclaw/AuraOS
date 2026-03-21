extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct RelationshipContextAware {
    entities: Vec<String>,
    relationships: Vec<(usize, usize)>,
}

impl RelationshipContextAware {
    pub fn new() -> Self {
        RelationshipContextAware {
            entities: Vec::new(),
            relationships: Vec::new(),
        }
    }

    pub fn add_entity(&mut self, entity_name: &str) -> usize {
        let index = self.entities.len();
        self.entities.push(String::from(entity_name));
        index
    }

    pub fn remove_entity(&mut self, entity_index: usize) {
        if entity_index < self.entities.len() {
            self.entities.remove(entity_index);
            self.relationships.retain(|&(a, b)| a != entity_index && b != entity_index);
        }
    }

    pub fn add_relationship(&mut self, entity1_index: usize, entity2_index: usize) {
        if entity1_index < self.entities.len() && entity2_index < self.entities.len() {
            self.relationships.push((entity1_index, entity2_index));
        }
    }

    pub fn remove_relationship(&mut self, entity1_index: usize, entity2_index: usize) {
        self.relationships.retain(|&(a, b)| a != entity1_index || b != entity2_index);
    }

    pub fn get_entities(&self) -> &Vec<String> {
        &self.entities
    }

    pub fn get_relationships(&self) -> &Vec<(usize, usize)> {
        &self.relationships
    }
}
