extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[allow(non_camel_case_types)]
pub struct relationship_important_people {
    name: String,
    age: u32,
    relationships: Vec<String>,
}

impl relationship_important_people {
    pub fn new(name: &str, age: u32) -> Self {
        relationship_important_people {
            name: String::from(name),
            age,
            relationships: Vec::new(),
        }
    }

    pub fn add_relationship(&mut self, person: &str) {
        self.relationships.push(String::from(person));
    }

    pub fn remove_relationship(&mut self, person: &str) -> bool {
        if let Some(index) = self.relationships.iter().position(|x| x == person) {
            self.relationships.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_age(&self) -> u32 {
        self.age
    }

    pub fn list_relationships(&self) -> &[String] {
        &self.relationships
    }
}
