extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct PeopleSourceGiftSent {
    people: Vec<String>,
    gifts: Vec<String>,
}

impl PeopleSourceGiftSent {
    pub fn new() -> Self {
        PeopleSourceGiftSent {
            people: Vec::new(),
            gifts: Vec::new(),
        }
    }

    pub fn add_person(&mut self, name: String) {
        if !self.people.contains(&name) {
            self.people.push(name);
        }
    }

    pub fn remove_person(&mut self, name: &str) -> bool {
        let index = self.people.iter().position(|x| x == name);
        match index {
            Some(i) => {
                self.people.remove(i);
                true
            },
            None => false,
        }
    }

    pub fn add_gift(&mut self, gift: String) {
        if !self.gifts.contains(&gift) {
            self.gifts.push(gift);
        }
    }

    pub fn remove_gift(&mut self, gift: &str) -> bool {
        let index = self.gifts.iter().position(|x| x == gift);
        match index {
            Some(i) => {
                self.gifts.remove(i);
                true
            },
            None => false,
        }
    }

    pub fn list_people(&self) -> Vec<String> {
        self.people.clone()
    }

    pub fn list_gifts(&self) -> Vec<String> {
        self.gifts.clone()
    }
}