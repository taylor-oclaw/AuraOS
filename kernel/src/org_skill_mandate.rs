extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct SkillMandate {
    title: String,
    description: String,
    required_skills: Vec<String>,
    priority_level: u8,
    active: bool,
}

impl SkillMandate {
    pub fn new(title: &str, description: &str, required_skills: &[&str], priority_level: u8) -> Self {
        SkillMandate {
            title: String::from(title),
            description: String::from(description),
            required_skills: required_skills.iter().map(|s| String::from(*s)).collect(),
            priority_level,
            active: true,
        }
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_required_skills(&self) -> &[String] {
        &self.required_skills
    }

    pub fn get_priority_level(&self) -> u8 {
        self.priority_level
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn add_required_skill(&mut self, skill: &str) {
        self.required_skills.push(String::from(skill));
    }

    pub fn remove_required_skill(&mut self, skill: &str) -> bool {
        if let Some(index) = self.required_skills.iter().position(|s| s == skill) {
            self.required_skills.remove(index);
            true
        } else {
            false
        }
    }

    pub fn update_description(&mut self, new_description: &str) {
        self.description = String::from(new_description);
    }

    pub fn update_priority_level(&mut self, new_priority_level: u8) {
        self.priority_level = new_priority_level;
    }
}
