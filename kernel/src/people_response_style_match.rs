extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct PeopleResponseStyleMatch {
    people: Vec<String>,
}

impl PeopleResponseStyleMatch {
    pub fn new() -> Self {
        PeopleResponseStyleMatch {
            people: Vec::new(),
        }
    }

    pub fn add_person(&mut self, name: String) {
        self.people.push(name);
    }

    pub fn remove_person(&mut self, name: &str) -> bool {
        if let Some(index) = self.people.iter().position(|x| x == name) {
            self.people.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_people_count(&self) -> usize {
        self.people.len()
    }

    pub fn find_person(&self, name: &str) -> Option<&String> {
        self.people.iter().find(|&&x| x == name)
    }

    pub fn list_people(&self) -> Vec<String> {
        self.people.clone()
    }
}
