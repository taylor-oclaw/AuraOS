extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct SkillLoopHandler {
    skills: Vec<String>,
}

impl SkillLoopHandler {
    pub fn new() -> Self {
        SkillLoopHandler {
            skills: Vec::new(),
        }
    }

    pub fn add_skill(&mut self, skill: String) {
        self.skills.push(skill);
    }

    pub fn remove_skill(&mut self, index: usize) -> Option<String> {
        if index < self.skills.len() {
            Some(self.skills.remove(index))
        } else {
            None
        }
    }

    pub fn get_skill(&self, index: usize) -> Option<&String> {
        self.skills.get(index)
    }

    pub fn list_skills(&self) -> &[String] {
        &self.skills
    }

    pub fn clear_skills(&mut self) {
        self.skills.clear();
    }
}
