extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

struct SkillMarketplace {
    skills: Vec<String>,
}

impl SkillMarketplace {
    pub fn new() -> Self {
        SkillMarketplace {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skill_marketplace() {
        let mut marketplace = SkillMarketplace::new();
        assert_eq!(marketplace.count_skills(), 0);

        marketplace.add_skill(String::from("AI Development"));
        marketplace.add_skill(String::from("Machine Learning"));

        assert_eq!(marketplace.count_skills(), 2);
        assert!(marketplace.has_skill("AI Development"));
        assert!(!marketplace.has_skill("Blockchain"));

        let skills = marketplace.list_skills();
        assert_eq!(skills.len(), 2);
        assert!(skills.contains(&String::from("AI Development")));
        assert!(skills.contains(&String::from("Machine Learning")));

        assert!(marketplace.remove_skill("Machine Learning"));
        assert!(!marketplace.has_skill("Machine Learning"));
        assert_eq!(marketplace.count_skills(), 1);

        assert!(!marketplace.remove_skill("Blockchain"));
    }
}
