extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn mam_mandatory_skill_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn mam_mandatory_skill_exit() {
    // Cleanup logic for the module
}

pub struct SkillManager {
    skills: Vec<String>,
}

impl SkillManager {
    pub fn new() -> Self {
        SkillManager {
            skills: Vec::new(),
        }
    }

    pub fn add_skill(&mut self, skill: String) {
        self.skills.push(skill);
    }

    pub fn remove_skill(&mut self, skill_name: &str) -> bool {
        if let Some(index) = self.skills.iter().position(|s| s == skill_name) {
            self.skills.remove(index);
            true
        } else {
            false
        }
    }

    pub fn has_skill(&self, skill_name: &str) -> bool {
        self.skills.contains(&String::from(skill_name))
    }

    pub fn list_skills(&self) -> Vec<String> {
        self.skills.clone()
    }

    pub fn count_skills(&self) -> usize {
        self.skills.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skill_manager() {
        let mut manager = SkillManager::new();
        assert_eq!(manager.count_skills(), 0);

        manager.add_skill(String::from("Rust"));
        manager.add_skill(String::from("AI"));
        assert_eq!(manager.count_skills(), 2);
        assert!(manager.has_skill("Rust"));
        assert!(!manager.has_skill("Python"));

        let skills = manager.list_skills();
        assert_eq!(skills, vec![String::from("Rust"), String::from("AI")]);

        assert!(manager.remove_skill("Rust"));
        assert!(!manager.remove_skill("Rust"));
        assert_eq!(manager.count_skills(), 1);
    }
}
