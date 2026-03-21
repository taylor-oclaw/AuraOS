extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct PeopleDetailOpinion {
    name: String,
    age: u32,
    opinions: Vec<String>,
}

impl PeopleDetailOpinion {
    pub fn new(name: &str, age: u32) -> Self {
        PeopleDetailOpinion {
            name: String::from(name),
            age,
            opinions: Vec::new(),
        }
    }

    pub fn add_opinion(&mut self, opinion: &str) {
        self.opinions.push(String::from(opinion));
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_age(&self) -> u32 {
        self.age
    }

    pub fn get_opinions(&self) -> &[String] {
        &self.opinions
    }

    pub fn has_opinion(&self, opinion: &str) -> bool {
        self.opinions.iter().any(|o| o == opinion)
    }
}
