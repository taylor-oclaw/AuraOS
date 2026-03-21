extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct PeopleMentionDetect {
    people: Vec<String>,
}

impl PeopleMentionDetect {
    pub fn new() -> Self {
        PeopleMentionDetect {
            people: Vec::new(),
        }
    }

    pub fn add_person(&mut self, name: &str) {
        self.people.push(String::from(name));
    }

    pub fn remove_person(&mut self, name: &str) {
        if let Some(index) = self.people.iter().position(|x| x == name) {
            self.people.remove(index);
        }
    }

    pub fn is_person_mentioned(&self, text: &str) -> bool {
        for person in &self.people {
            if text.contains(person) {
                return true;
            }
        }
        false
    }

    pub fn list_people(&self) -> Vec<String> {
        self.people.clone()
    }

    pub fn count_people(&self) -> usize {
        self.people.len()
    }
}
