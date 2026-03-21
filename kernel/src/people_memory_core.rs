extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn people_memory_core_init() {
    // Initialization logic for the module
}

pub extern "C" fn people_memory_core_exit() {
    // Cleanup logic for the module
}

pub struct Person {
    name: String,
    age: u32,
    email: String,
}

impl Person {
    pub fn new(name: &str, age: u32, email: &str) -> Self {
        Person {
            name: String::from(name),
            age,
            email: String::from(email),
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

    pub fn get_email(&self) -> &str {
        &self.email
    }

    pub fn set_email(&mut self, email: &str) {
        self.email = String::from(email);
    }
}

pub struct PeopleMemoryCore {
    people: Vec<Person>,
}

impl PeopleMemoryCore {
    pub fn new() -> Self {
        PeopleMemoryCore {
            people: Vec::new(),
        }
    }

    pub fn add_person(&mut self, person: Person) {
        self.people.push(person);
    }

    pub fn remove_person_by_name(&mut self, name: &str) {
        self.people.retain(|p| p.get_name() != name);
    }

    pub fn get_person_by_name(&self, name: &str) -> Option<&Person> {
        self.people.iter().find(|p| p.get_name() == name)
    }

    pub fn list_people(&self) -> Vec<String> {
        self.people.iter().map(|p| String::from("info"), p.get_age(), p.get_email())).collect(
    }
}
