extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct OfflineSkillCache {
    skills: Vec<String>,
}

impl OfflineSkillCache {
    pub fn new() -> Self {
        OfflineSkillCache {
            skills: Vec::new(),
        }
    }

    pub fn add_skill(&mut self, skill: String) {
        if !self.skills.contains(&skill) {
            self.skills.push(skill);
        }
    }

    pub fn remove_skill(&mut self, skill: &str) -> bool {
        let index = self.skills.iter().position(|s| s == skill);
        if let Some(i) = index {
            self.skills.remove(i);
            true
        } else {
            false
        }
    }

    pub fn has_skill(&self, skill: &str) -> bool {
        self.skills.contains(&String::from(skill))
    }

    pub fn list_skills(&self) -> Vec<String> {
        self.skills.clone()
    }

    pub fn clear_skills(&mut self) {
        self.skills.clear();
    }
}
