extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct PeopleMemoryContextTrigger {
    people: Vec<String>,
}

impl PeopleMemoryContextTrigger {
    pub fn new() -> Self {
        PeopleMemoryContextTrigger {
            people: Vec::new(),
        }
    }

    pub fn add_person(&mut self, name: String) {
        self.people.push(name);
    }

    pub fn remove_person(&mut self, index: usize) -> Option<String> {
        if index < self.people.len() {
            Some(self.people.remove(index))
        } else {
            None
        }
    }

    pub fn get_person(&self, index: usize) -> Option<&String> {
        self.people.get(index)
    }

    pub fn list_people(&self) -> &Vec<String> {
        &self.people
    }

    pub fn count_people(&self) -> usize {
        self.people.len()
    }
}
