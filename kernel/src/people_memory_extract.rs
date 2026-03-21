extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct PeopleMemoryExtract {
    people: Vec<Person>,
}

impl PeopleMemoryExtract {
    pub fn new() -> Self {
        PeopleMemoryExtract {
            people: Vec::new(),
        }
    }

    pub fn add_person(&mut self, name: String, age: u8) {
        let person = Person { name, age };
        self.people.push(person);
    }

    pub fn get_person_by_name(&self, name: &str) -> Option<&Person> {
        self.people.iter().find(|p| p.name == name)
    }

    pub fn remove_person_by_name(&mut self, name: &str) -> bool {
        let pos = self.people.iter().position(|p| p.name == name);
        if let Some(index) = pos {
            self.people.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_average_age(&self) -> Option<u8> {
        if self.people.is_empty() {
            None
        } else {
            let total_age: u32 = self.people.iter().map(|p| p.age as u32).sum();
            Some((total_age / self.people.len() as u32) as u8)
        }
    }

    pub fn list_people(&self) -> Vec<&Person> {
        self.people.iter().collect()
    }
}

struct Person {
    name: String,
    age: u8,
}
