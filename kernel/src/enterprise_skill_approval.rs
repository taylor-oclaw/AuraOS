extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct EnterpriseSkillApproval {
    skills: Vec<String>,
    approved_skills: Vec<String>,
}

impl EnterpriseSkillApproval {
    pub fn new() -> Self {
        EnterpriseSkillApproval {
            skills: Vec::new(),
            approved_skills: Vec::new(),
        }
    }

    pub fn add_skill(&mut self, skill: String) {
        if !self.skills.contains(&skill) {
            self.skills.push(skill);
        }
    }

    pub fn approve_skill(&mut self, skill: &str) -> bool {
        if let Some(index) = self.skills.iter().position(|s| s == skill) {
            let approved_skill = self.skills.remove(index);
            self.approved_skills.push(approved_skill);
            true
        } else {
            false
        }
    }

    pub fn is_skill_approved(&self, skill: &str) -> bool {
        self.approved_skills.contains(&skill.to_string())
    }

    pub fn list_pending_skills(&self) -> Vec<String> {
        self.skills.clone()
    }

    pub fn list_approved_skills(&self) -> Vec<String> {
        self.approved_skills.clone()
    }
}
