extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn mam_optional_skill_init() {
    // Initialization logic for the module
}

pub extern "C" fn mam_optional_skill_exit() {
    // Cleanup logic for the module
}

pub struct MamOptionalSkill {
    name: String,
    description: String,
    skills: Vec<String>,
    level: u32,
    active: bool,
}

impl MamOptionalSkill {
    pub fn new(name: &str, description: &str) -> Self {
        MamOptionalSkill {
            name: String::from(name),
            description: String::from(description),
            skills: Vec::new(),
            level: 1,
            active: false,
        }
    }

    pub fn add_skill(&mut self, skill: &str) {
        self.skills.push(String::from(skill));
    }

    pub fn remove_skill(&mut self, skill: &str) -> bool {
        if let Some(index) = self.skills.iter().position(|s| s == skill) {
            self.skills.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_skills(&self) -> &[String] {
        &self.skills
    }

    pub fn set_level(&mut self, level: u32) {
        self.level = level;
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mam_optional_skill() {
        let mut skill = MamOptionalSkill::new("AI Skill", "A skill for AI operations");
        assert_eq!(skill.name, "AI Skill");
        assert_eq!(skill.description, "A skill for AI operations");
        assert_eq!(skill.skills.len(), 0);
        assert_eq!(skill.level, 1);
        assert!(!skill.is_active());

        skill.add_skill("Machine Learning");
        skill.add_skill("Natural Language Processing");
        assert_eq!(skill.get_skills().len(), 2);

        assert!(skill.remove_skill("Machine Learning"));
        assert_eq!(skill.get_skills().len(), 1);

        skill.set_level(5);
        assert_eq!(skill.level, 5);

        skill.activate();
        assert!(skill.is_active());

        skill.deactivate();
        assert!(!skill.is_active());
    }
}
