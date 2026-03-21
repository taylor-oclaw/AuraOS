extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct PeopleDetailRecommendation {
    people: Vec<Person>,
}

impl PeopleDetailRecommendation {
    pub fn new() -> Self {
        PeopleDetailRecommendation {
            people: Vec::new(),
        }
    }

    pub fn add_person(&mut self, name: String, age: u8, occupation: String) {
        let person = Person { name, age, occupation };
        self.people.push(person);
    }

    pub fn get_people_count(&self) -> usize {
        self.people.len()
    }

    pub fn find_by_name(&self, name: &str) -> Option<&Person> {
        self.people.iter().find(|p| p.name == name)
    }

    pub fn recommend_person(&self, age_limit: u8, occupation_filter: &str) -> Vec<&Person> {
        self.people
            .iter()
            .filter(|p| p.age <= age_limit && p.occupation == occupation_filter)
            .collect()
    }
}

struct Person {
    name: String,
    age: u8,
    occupation: String,
}
