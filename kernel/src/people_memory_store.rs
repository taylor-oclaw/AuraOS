extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct PeopleMemoryStore {
    people: Vec<Person>,
}

impl PeopleMemoryStore {
    pub fn new() -> Self {
        PeopleMemoryStore {
            people: Vec::new(),
        }
    }

    pub fn add_person(&mut self, name: String, age: u32) {
        let person = Person { name, age };
        self.people.push(person);
    }

    pub fn get_person_by_name(&self, name: &str) -> Option<&Person> {
        self.people.iter().find(|p| p.name == name)
    }

    pub fn remove_person_by_name(&mut self, name: &str) {
        self.people.retain(|p| p.name != name);
    }

    pub fn get_all_people(&self) -> &[Person] {
        &self.people
    }

    pub fn count_people(&self) -> usize {
        self.people.len()
    }
}

pub struct Person {
    name: String,
    age: u32,
}
