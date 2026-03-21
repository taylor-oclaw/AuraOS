extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

struct Person {
    name: String,
    available: bool,
}

impl Person {
    pub fn new(name: &str, available: bool) -> Self {
        Person {
            name: String::from(name),
            available,
        }
    }

    pub fn is_available(&self) -> bool {
        self.available
    }

    pub fn set_availability(&mut self, available: bool) {
        self.available = available;
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn toggle_availability(&mut self) {
        self.available = !self.available;
    }
}

struct PeopleAvailability {
    people: Vec<Person>,
}

impl PeopleAvailability {
    pub fn new() -> Self {
        PeopleAvailability {
            people: Vec::new(),
        }
    }

    pub fn add_person(&mut self, name: &str, available: bool) {
        let person = Person::new(name, available);
        self.people.push(person);
    }

    pub fn remove_person_by_name(&mut self, name: &str) {
        self.people.retain(|p| p.get_name() != name);
    }

    pub fn get_all_people(&self) -> Vec<&Person> {
        self.people.iter().collect()
    }

    pub fn find_person_by_name(&self, name: &str) -> Option<&Person> {
        self.people.iter().find(|&p| p.get_name() == name)
    }
}
