extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct Profile {
    name: String,
    age: u32,
    skills: Vec<String>,
    experience_years: u32,
    is_active: bool,
}

impl Profile {
    pub fn new(name: &str, age: u32) -> Self {
        Profile {
            name: String::from(name),
            age,
            skills: Vec::new(),
            experience_years: 0,
            is_active: true,
        }
    }

    pub fn add_skill(&mut self, skill: &str) {
        self.skills.push(String::from(skill));
    }

    pub fn remove_skill(&mut self, skill: &str) -> bool {
        if let Some(index) = self.skills.iter().position(|s| s == skill) {
            self.skills.remove(index);
            true
        } else {
            false
        }
    }

    pub fn set_experience_years(&mut self, years: u32) {
        self.experience_years = years;
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    pub fn get_profile_info(&self) -> String {
        let mut info = format!("Name: {}, Age: {}\n", self.name, self.age);
        info.push_str("Skills:\n");
        for skill in &self.skills {
            info.push_str(format!("- {}\n", skill).as_str());
        }
        info.push_str(&format!("Experience Years: {}\n", self.experience_years));
        info.push_str(&format!("Active: {}\n", if self.is_active { "Yes" } else { "No" }));
        info
    }
}
