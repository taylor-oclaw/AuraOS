extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

pub struct SkillScopePublic {
    skills: Vec<String>,
    level: u8,
}

impl SkillScopePublic {
    pub fn new(level: u8) -> Self {
        SkillScopePublic {
            skills: Vec::new(),
            level,
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

    pub fn get_skills(&self) -> &Vec<String> {
        &self.skills
    }

    pub fn set_level(&mut self, level: u8) {
        self.level = level;
    }

    pub fn get_level(&self) -> u8 {
        self.level
    }
}
