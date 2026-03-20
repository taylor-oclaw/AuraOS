extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MedusaV2 {
    name: String,
    age: u32,
    abilities: Vec<String>,
}

impl MedusaV2 {
    pub fn new(name: &str, age: u32) -> Self {
        MedusaV2 {
            name: String::from(name),
            age,
            abilities: Vec::new(),
        }
    }

    pub fn add_ability(&mut self, ability: &str) {
        self.abilities.push(String::from(ability));
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_age(&self) -> u32 {
        self.age
    }

    pub fn list_abilities(&self) -> &[String] {
        &self.abilities
    }

    pub fn remove_ability(&mut self, ability: &str) {
        if let Some(index) = self.abilities.iter().position(|a| a == ability) {
            self.abilities.remove(index);
        }
    }
}
