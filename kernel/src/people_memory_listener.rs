extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct PeopleMemoryListener {
    people: Vec<String>,
}

impl PeopleMemoryListener {
    pub fn new() -> Self {
        PeopleMemoryListener {
            people: Vec::new(),
        }
    }

    pub fn add_person(&mut self, name: String) {
        self.people.push(name);
    }

    pub fn remove_person(&mut self, name: &str) -> bool {
        if let Some(index) = self.people.iter().position(|person| person == name) {
            self.people.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_people_count(&self) -> usize {
        self.people.len()
    }

    pub fn list_people(&self) -> Vec<String> {
        self.people.clone()
    }

    pub fn find_person(&self, name: &str) -> Option<&String> {
        self.people.iter().find(|person| person == &name)
    }
}
