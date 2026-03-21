extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct Person {
    name: String,
    age: u8,
}

impl Person {
    pub fn new(name: &str, age: u8) -> Self {
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

    pub fn get_age(&self) -> u8 {
        self.age
    }

    pub fn set_age(&mut self, age: u8) {
        self.age = age;
    }
}

#[derive(Debug)]
pub struct TravelPlan {
    person: Person,
    destinations: Vec<String>,
}

impl TravelPlan {
    pub fn new(person: Person, destinations: Vec<&str>) -> Self {
        let dest_vec: Vec<String> = destinations.into_iter().map(String::from).collect();
        TravelPlan {
            person,
            destinations: dest_vec,
        }
    }

    pub fn get_person(&self) -> &Person {
        &self.person
    }

    pub fn set_person(&mut self, person: Person) {
        self.person = person;
    }

    pub fn add_destination(&mut self, destination: &str) {
        self.destinations.push(String::from(destination));
    }

    pub fn remove_destination(&mut self, index: usize) -> Option<String> {
        if index < self.destinations.len() {
            Some(self.destinations.remove(index))
        } else {
            None
        }
    }

    pub fn get_destinations(&self) -> &Vec<String> {
        &self.destinations
    }
}
