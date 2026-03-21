extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct Person {
    name: String,
    pronoun: String,
}

impl Person {
    pub fn new(name: &str, pronoun: &str) -> Self {
        Person {
            name: String::from(name),
            pronoun: String::from(pronoun),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, new_name: &str) {
        self.name = String::from(new_name);
    }

    pub fn get_pronoun(&self) -> &str {
        &self.pronoun
    }

    pub fn set_pronoun(&mut self, new_pronoun: &str) {
        self.pronoun = String::from(new_pronoun);
    }

    pub fn introduce(&self) -> String {
        String::from("info")
    }
}

#[derive(Debug)]
pub struct PeoplePronounRemember {
    people: Vec<Person>,
}

impl PeoplePronounRemember {
    pub fn new() -> Self {
        PeoplePronounRemember {
            people: Vec::new(),
        }
    }

    pub fn add_person(&mut self, name: &str, pronoun: &str) {
        let person = Person::new(name, pronoun);
        self.people.push(person);
    }

    pub fn get_people_count(&self) -> usize {
        self.people.len()
    }

    pub fn find_person_by_name(&self, name: &str) -> Option<&Person> {
        self.people.iter().find(|p| p.get_name() == name)
    }

    pub fn list_all_people(&self) -> Vec<String> {
        self.people.iter().map(|p| p.introduce()).collect()
    }
}
