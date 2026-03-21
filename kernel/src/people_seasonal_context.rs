extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct PeopleSeasonalContext {
    people: Vec<String>,
    season: String,
}

impl PeopleSeasonalContext {
    pub fn new(season: &str) -> Self {
        PeopleSeasonalContext {
            people: Vec::new(),
            season: String::from(season),
        }
    }

    pub fn add_person(&mut self, name: &str) {
        self.people.push(String::from(name));
    }

    pub fn remove_person(&mut self, name: &str) -> bool {
        if let Some(index) = self.people.iter().position(|x| x == name) {
            self.people.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_people_count(&self) -> usize {
        self.people.len()
    }

    pub fn is_person_present(&self, name: &str) -> bool {
        self.people.contains(&String::from(name))
    }

    pub fn change_season(&mut self, new_season: &str) {
        self.season = String::from(new_season);
    }
}
