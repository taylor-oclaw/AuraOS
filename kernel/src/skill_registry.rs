extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct SkillRegistry {
    skills: Vec<String>,
}

impl SkillRegistry {
    pub fn new() -> Self {
        SkillRegistry {
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
        match index {
            Some(i) => {
                self.skills.remove(i);
                true
            }
            None => false,
        }
    }

    pub fn has_skill(&self, skill: &str) -> bool {
        self.skills.contains(&String::from(skill))
    }

    pub fn list_skills(&self) -> Vec<String> {
        self.skills.clone()
    }

    pub fn count_skills(&self) -> usize {
        self.skills.len()
    }
}
