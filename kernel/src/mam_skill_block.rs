extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn mam_skill_block_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn mam_skill_block_exit() {
    // Cleanup logic for the module
}

pub struct MamSkillBlock {
    skills: Vec<String>,
    active_skill: Option<usize>,
}

impl MamSkillBlock {
    pub fn new() -> Self {
        MamSkillBlock {
            skills: Vec::new(),
            active_skill: None,
        }
    }

    pub fn add_skill(&mut self, skill_name: &str) {
        self.skills.push(String::from(skill_name));
    }

    pub fn remove_skill(&mut self, index: usize) -> Option<String> {
        if index < self.skills.len() {
            Some(self.skills.remove(index))
        } else {
            None
        }
    }

    pub fn list_skills(&self) -> Vec<&str> {
        self.skills.iter().map(|s| s.as_str()).collect()
    }

    pub fn activate_skill(&mut self, index: usize) -> bool {
        if index < self.skills.len() {
            self.active_skill = Some(index);
            true
        } else {
            false
        }
    }

    pub fn get_active_skill(&self) -> Option<&str> {
        self.active_skill.map(|index| &self.skills[index])
    }
}
