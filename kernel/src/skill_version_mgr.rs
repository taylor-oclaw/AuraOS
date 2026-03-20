extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct SkillVersionMgr {
    skills: Vec<(String, String)>,
}

impl SkillVersionMgr {
    pub fn new() -> Self {
        SkillVersionMgr {
            skills: Vec::new(),
        }
    }

    pub fn add_skill(&mut self, skill_name: &str, version: &str) {
        let skill = (String::from(skill_name), String::from(version));
        self.skills.push(skill);
    }

    pub fn get_version(&self, skill_name: &str) -> Option<&str> {
        for (name, version) in &self.skills {
            if name == skill_name {
                return Some(version);
            }
        }
        None
    }

    pub fn update_version(&mut self, skill_name: &str, new_version: &str) -> bool {
        for skill in &mut self.skills {
            if skill.0 == skill_name {
                skill.1 = String::from(new_version);
                return true;
            }
        }
        false
    }

    pub fn remove_skill(&mut self, skill_name: &str) -> bool {
        let pos = self.skills.iter().position(|(name, _)| name == skill_name);
        if let Some(index) = pos {
            self.skills.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_skills(&self) -> Vec<(String, String)> {
        self.skills.clone()
    }
}
