extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct SkillDiscovery {
    skills: Vec<String>,
}

impl SkillDiscovery {
    pub fn new() -> Self {
        SkillDiscovery {
            skills: Vec::new(),
        }
    }

    pub fn add_skill(&mut self, skill: String) {
        self.skills.push(skill);
    }

    pub fn remove_skill(&mut self, skill: &str) -> bool {
        if let Some(index) = self.skills.iter().position(|s| s == skill) {
            self.skills.remove(index);
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

    pub fn count_skills(&self) -> usize {
        self.skills.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skill_discovery() {
        let mut sd = SkillDiscovery::new();
        assert_eq!(sd.count_skills(), 0);

        sd.add_skill(String::from("Rust"));
        assert_eq!(sd.count_skills(), 1);
        assert!(sd.has_skill("Rust"));

        sd.add_skill(String::from("AI"));
        assert_eq!(sd.count_skills(), 2);
        assert!(sd.has_skill("AI"));

        let skills = sd.list_skills();
        assert_eq!(skills.len(), 2);
        assert!(skills.contains(&String::from("Rust")));
        assert!(skills.contains(&String::from("AI")));

        assert!(sd.remove_skill("Rust"));
        assert!(!sd.has_skill("Rust"));
        assert_eq!(sd.count_skills(), 1);

        assert!(!sd.remove_skill("Python"));
        assert_eq!(sd.count_skills(), 1);
    }
}
