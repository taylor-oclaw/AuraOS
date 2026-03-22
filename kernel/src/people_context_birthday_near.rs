extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct PeopleContextBirthdayNear {
    people: Vec<Person>,
}

impl PeopleContextBirthdayNear {
    pub fn new() -> Self {
        PeopleContextBirthdayNear {
            people: Vec::new(),
        }
    }

    pub fn add_person(&mut self, name: String, birthday: u32) {
        let person = Person { name, birthday };
        self.people.push(person);
    }

    pub fn remove_person_by_name(&mut self, name: &str) -> bool {
        if let Some(index) = self.people.iter().position(|p| p.name == name) {
            self.people.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_people_with_birthday_near(&self, days: u32) -> Vec<&Person> {
        let current_day = 100; // Assume today is day 100 for simplicity
        self.people
            .iter()
            .filter(|p| (p.birthday - current_day).abs() <= days)
            .collect()
    }

    pub fn get_person_by_name(&self, name: &str) -> Option<&Person> {
        self.people.iter().find(|p| p.name == name)
    }

    pub fn count_people(&self) -> usize {
        self.people.len()
    }
}

struct Person {
    name: String,
    birthday: u32, // Day of the year (1-365)
}