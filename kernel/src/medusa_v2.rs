extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn medusa_v2_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn medusa_v2_exit() {
    // Cleanup logic for the module
}

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

    pub fn remove_ability(&mut self, ability: &str) -> bool {
        if let Some(index) = self.abilities.iter().position(|a| a == ability) {
            self.abilities.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_age(&self) -> u32 {
        self.age
    }

    pub fn list_abilities(&self) -> Vec<&str> {
        self.abilities.iter().map(|a| a.as_str()).collect()
    }
}