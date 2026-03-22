#![no_std]
#![feature(allocator_api)]
#![feature(const_mut_refs)]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct PeopleSourcePhoneCall {
    people: Vec<String>,
}

impl PeopleSourcePhoneCall {
    pub fn new() -> Self {
        PeopleSourcePhoneCall { people: Vec::new() }
    }

    pub fn add_person(&mut self, name: String) {
        self.people.push(name);
    }

    pub fn remove_person(&mut self, name: &str) -> Option<String> {
        if let Some(index) = self.people.iter().position(|p| p == name) {
            Some(self.people.remove(index))
        } else {
            None
        }
    }

    pub fn call_person(&self, name: &str) -> Option<&String> {
        self.people.iter().find(|p| p == name)
    }

    pub fn list_people(&self) -> Vec<String> {
        self.people.clone()
    }
}