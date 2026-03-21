extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct FamilyHubAgeGate {
    min_age: u8,
    max_age: u8,
    allowed_ages: Vec<u8>,
}

impl FamilyHubAgeGate {
    pub fn new(min_age: u8, max_age: u8) -> Self {
        let mut allowed_ages = Vec::new();
        for age in min_age..=max_age {
            allowed_ages.push(age);
        }
        FamilyHubAgeGate {
            min_age,
            max_age,
            allowed_ages,
        }
    }

    pub fn is_age_allowed(&self, age: u8) -> bool {
        self.allowed_ages.contains(&age)
    }

    pub fn add_allowed_age(&mut self, age: u8) {
        if age >= self.min_age && age <= self.max_age {
            self.allowed_ages.push(age);
        }
    }

    pub fn remove_allowed_age(&mut self, age: u8) {
        if let Some(index) = self.allowed_ages.iter().position(|&a| a == age) {
            self.allowed_ages.remove(index);
        }
    }

    pub fn get_min_age(&self) -> u8 {
        self.min_age
    }

    pub fn get_max_age(&self) -> u8 {
        self.max_age
    }
}
