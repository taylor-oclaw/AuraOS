extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct PeopleContextNotStale {
    people: Vec<String>,
}

impl PeopleContextNotStale {
    pub fn new() -> Self {
        PeopleContextNotStale { people: Vec::new() }
    }

    pub fn add_person(&mut self, name: String) {
        if !self.people.contains(&name) {
            self.people.push(name);
        }
    }

    pub fn remove_person(&mut self, name: &String) -> bool {
        let index = self.people.iter().position(|p| p == name);
        match index {
            Some(i) => {
                self.people.remove(i);
                true
            }
            None => false,
        }
    }

    pub fn get_people_count(&self) -> usize {
        self.people.len()
    }

    pub fn is_person_stale(&self, name: &String) -> bool {
        let index = self.people.iter().position(|p| p == name);
        match index {
            Some(_) => false,
            None => true,
        }
    }

    pub fn get_people_list(&self) -> Vec<String> {
        self.people.clone()
    }
}