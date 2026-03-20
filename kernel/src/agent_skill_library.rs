extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

mod agent_skill_library {
    use super::*;

    pub struct SkillLibrary {
        skills: Vec<String>,
    }

    impl SkillLibrary {
        pub fn new() -> Self {
            SkillLibrary {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skill_library() {
        let mut library = agent_skill_library::SkillLibrary::new();
        assert_eq!(library.count_skills(), 0);

        library.add_skill("Programming");
        assert_eq!(library.count_skills(), 1);
        assert!(library.has_skill("Programming"));

        library.add_skill("Machine Learning");
        assert_eq!(library.count_skills(), 2);
        assert!(library.has_skill("Machine Learning"));

        let skills = library.list_skills();
        assert_eq!(skills.len(), 2);
        assert!(skills.contains(&String::from("Programming")));
        assert!(skills.contains(&String::from("Machine Learning")));

        library.remove_skill("Programming");
        assert_eq!(library.count_skills(), 1);
        assert!(!library.has_skill("Programming"));

        library.remove_skill("Machine Learning");
        assert_eq!(library.count_skills(), 0);
        assert!(!library.has_skill("Machine Learning"));
    }
}
