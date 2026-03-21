extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct PeopleTimezoneRemember {
    people: Vec<Person>,
}

impl PeopleTimezoneRemember {
    pub fn new() -> Self {
        PeopleTimezoneRemember {
            people: Vec::new(),
        }
    }

    pub fn add_person(&mut self, name: String, timezone: String) {
        let person = Person { name, timezone };
        self.people.push(person);
    }

    pub fn get_timezone(&self, name: &str) -> Option<&String> {
        self.people.iter().find(|p| p.name == name).map(|p| &p.timezone)
    }

    pub fn remove_person(&mut self, name: &str) {
        self.people.retain(|p| p.name != name);
    }

    pub fn list_people(&self) -> Vec<&String> {
        self.people.iter().map(|p| &p.name).collect()
    }

    pub fn update_timezone(&mut self, name: &str, new_timezone: String) -> bool {
        if let Some(person) = self.people.iter_mut().find(|p| p.name == name) {
            person.timezone = new_timezone;
            true
        } else {
            false
        }
    }
}

struct Person {
    name: String,
    timezone: String,
}
