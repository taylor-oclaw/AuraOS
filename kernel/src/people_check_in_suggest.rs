extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct PeopleCheckInSuggest {
    people: Vec<String>,
}

impl PeopleCheckInSuggest {
    pub fn new() -> Self {
        PeopleCheckInSuggest {
            people: Vec::new(),
        }
    }

    pub fn add_person(&mut self, name: String) {
        if !self.people.contains(&name) {
            self.people.push(name);
        }
    }

    pub fn remove_person(&mut self, name: &str) {
        self.people.retain(|person| person != name);
    }

    pub fn get_all_people(&self) -> Vec<String> {
        self.people.clone()
    }

    pub fn check_in_suggest(&self, name: &str) -> bool {
        self.people.contains(&String::from(name))
    }

    pub fn suggest_next_person(&self) -> Option<&String> {
        if self.people.is_empty() {
            None
        } else {
            Some(self.people.get(0).unwrap())
        }
    }
}
