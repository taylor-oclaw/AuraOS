extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn people_follow_up_natural_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn people_follow_up_natural_exit() {
    // Cleanup logic for the module
}

pub struct PeopleFollowUpNatural {
    names: Vec<String>,
    ages: Vec<u32>,
    interests: Vec<Vec<String>>,
}

impl PeopleFollowUpNatural {
    pub fn new() -> Self {
        PeopleFollowUpNatural {
            names: Vec::new(),
            ages: Vec::new(),
            interests: Vec::new(),
        }
    }

    pub fn add_person(&mut self, name: String, age: u32, interest: Vec<String>) {
        self.names.push(name);
        self.ages.push(age);
        self.interests.push(interest);
    }

    pub fn get_person_count(&self) -> usize {
        self.names.len()
    }

    pub fn get_person_age(&self, index: usize) -> Option<u32> {
        self.ages.get(index).cloned()
    }

    pub fn get_person_interests(&self, index: usize) -> Option<&Vec<String>> {
        self.interests.get(index)
    }

    pub fn find_people_by_interest(&self, interest: &str) -> Vec<usize> {
        let mut indices = Vec::new();
        for (i, interests) in self.interests.iter().enumerate() {
            if interests.contains(&String::from(interest)) {
                indices.push(i);
            }
        }
        indices
    }
}
