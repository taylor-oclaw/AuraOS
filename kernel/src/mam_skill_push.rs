extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn mam_skill_push_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn mam_skill_push_exit() {
    // Cleanup logic for the module
}

pub struct MamSkillPush {
    skills: Vec<String>,
}

impl MamSkillPush {
    pub fn new() -> Self {
        MamSkillPush {
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

    pub fn get_skills(&self) -> &[String] {
        &self.skills
    }

    pub fn has_skill(&self, skill: &str) -> bool {
        self.skills.iter().any(|s| s == skill)
    }

    pub fn clear_skills(&mut self) {
        self.skills.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mam_skill_push() {
        let mut mam = MamSkillPush::new();
        assert_eq!(mam.get_skills().len(), 0);

        mam.add_skill(String::from("AI"));
        mam.add_skill(String::from("Machine Learning"));
        assert_eq!(mam.get_skills().len(), 2);
        assert!(mam.has_skill("AI"));

        let removed = mam.remove_skill(0);
        assert_eq!(removed, Some(String::from("AI")));
        assert!(!mam.has_skill("AI"));

        mam.clear_skills();
        assert_eq!(mam.get_skills().len(), 0);
    }
}
