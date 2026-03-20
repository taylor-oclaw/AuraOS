extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct WasmSkillRuntime {
    skills: Vec<String>,
}

impl WasmSkillRuntime {
    pub fn new() -> Self {
        WasmSkillRuntime {
            skills: Vec::new(),
        }
    }

    pub fn add_skill(&mut self, skill_name: &str) {
        self.skills.push(String::from(skill_name));
    }

    pub fn remove_skill(&mut self, skill_name: &str) {
        if let Some(index) = self.skills.iter().position(|s| s == skill_name) {
            self.skills.remove(index);
        }
    }

    pub fn list_skills(&self) -> Vec<String> {
        self.skills.clone()
    }

    pub fn has_skill(&self, skill_name: &str) -> bool {
        self.skills.contains(&String::from(skill_name))
    }

    pub fn count_skills(&self) -> usize {
        self.skills.len()
    }
}
