extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct FamilyHubSkillLibrary {
    skills: Vec<String>,
}

impl FamilyHubSkillLibrary {
    pub fn new() -> Self {
        FamilyHubSkillLibrary {
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

    pub fn get_skill_count(&self) -> usize {
        self.skills.len()
    }

    pub fn list_skills(&self) -> Vec<String> {
        self.skills.clone()
    }

    pub fn has_skill(&self, skill: &str) -> bool {
        self.skills.iter().any(|s| s == skill)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_family_hub_skill_library() {
        let mut library = FamilyHubSkillLibrary::new();
        assert_eq!(library.get_skill_count(), 0);

        library.add_skill(String::from("Cooking"));
        library.add_skill(String::from("Gardening"));
        assert_eq!(library.get_skill_count(), 2);

        assert!(library.has_skill("Cooking"));
        assert!(!library.has_skill("Programming"));

        let skills = library.list_skills();
        assert_eq!(skills, vec![String::from("Cooking"), String::from("Gardening")]);

        let removed_skill = library.remove_skill(0);
        assert_eq!(removed_skill, Some(String::from("Cooking")));
        assert_eq!(library.get_skill_count(), 1);

        let skills_after_removal = library.list_skills();
        assert_eq!(skills_after_removal, vec![String::from("Gardening")]);
    }
}
