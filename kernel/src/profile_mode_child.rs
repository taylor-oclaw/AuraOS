extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ProfileModeChild {
    name: String,
    age: u32,
    skills: Vec<String>,
    active: bool,
}

impl ProfileModeChild {
    pub fn new(name: &str, age: u32) -> Self {
        ProfileModeChild {
            name: String::from(name),
            age,
            skills: Vec::new(),
            active: true,
        }
    }

    pub fn add_skill(&mut self, skill: &str) {
        self.skills.push(String::from(skill));
    }

    pub fn remove_skill(&mut self, skill: &str) -> bool {
        if let Some(pos) = self.skills.iter().position(|s| s == skill) {
            self.skills.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn list_skills(&self) -> Vec<String> {
        self.skills.clone()
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn activate(&mut self) {
        self.active = true;
    }
}
