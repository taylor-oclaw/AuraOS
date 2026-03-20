extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    println!("Rust module started!");
    0
}

pub struct SkillLifecycleMgr {
    skills: Vec<String>,
}

impl SkillLifecycleMgr {
    pub fn new() -> Self {
        SkillLifecycleMgr {
            skills: Vec::new(),
        }
    }

    pub fn add_skill(&mut self, skill_name: &str) {
        self.skills.push(String::from(skill_name));
    }

    pub fn remove_skill(&mut self, skill_name: &str) {
        if let Some(index) = self.skills.iter().position(|s| s == skill_name) {
            self.skills.remove(index);
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
    fn test_skill_lifecycle_mgr() {
        let mut mgr = SkillLifecycleMgr::new();
        assert_eq!(mgr.count_skills(), 0);

        mgr.add_skill("AI");
        assert_eq!(mgr.count_skills(), 1);
        assert!(mgr.has_skill("AI"));

        mgr.add_skill("Machine Learning");
        assert_eq!(mgr.count_skills(), 2);
        assert!(mgr.has_skill("Machine Learning"));

        let skills = mgr.list_skills();
        assert_eq!(skills.len(), 2);
        assert!(skills.contains(&String::from("AI")));
        assert!(skills.contains(&String::from("Machine Learning")));

        mgr.remove_skill("AI");
        assert_eq!(mgr.count_skills(), 1);
        assert!(!mgr.has_skill("AI"));

        mgr.remove_skill("Machine Learning");
        assert_eq!(mgr.count_skills(), 0);
        assert!(!mgr.has_skill("Machine Learning"));
    }
}
