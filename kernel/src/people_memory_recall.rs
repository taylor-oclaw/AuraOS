extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct Person {
    name: String,
    age: u32,
}

impl Person {
    pub fn new(name: &str, age: u32) -> Self {
        Person {
            name: String::from(name),
            age,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }

    pub fn get_age(&self) -> u32 {
        self.age
    }

    pub fn set_age(&mut self, age: u32) {
        self.age = age;
    }

    pub fn is_adult(&self) -> bool {
        self.age >= 18
    }
}

#[repr(C)]
pub struct PeopleMemoryRecall {
    people: Vec<Person>,
}

impl PeopleMemoryRecall {
    pub fn new() -> Self {
        PeopleMemoryRecall {
            people: Vec::new(),
        }
    }

    pub fn add_person(&mut self, person: Person) {
        self.people.push(person);
    }

    pub fn get_people_count(&self) -> usize {
        self.people.len()
    }

    pub fn find_person_by_name(&self, name: &str) -> Option<&Person> {
        self.people.iter().find(|p| p.get_name() == name)
    }

    pub fn remove_person_by_name(&mut self, name: &str) {
        self.people.retain(|p| p.get_name() != name);
    }
}
