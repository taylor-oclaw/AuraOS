extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_std]
mod skill_fan_in {
    use core::fmt::{Debug, Formatter};
    use core::ptr;

    #[derive(Debug)]
    pub struct SkillFanIn {
        skills: Vec<String>,
        active_skill_index: usize,
    }

    impl SkillFanIn {
        pub fn new() -> Self {
            SkillFanIn {
                skills: Vec::new(),
                active_skill_index: 0,
            }
        }

        pub fn add_skill(&mut self, skill_name: &str) {
            self.skills.push(String::from(skill_name));
        }

        pub fn remove_skill(&mut self, index: usize) -> Option<String> {
            if index < self.skills.len() {
                Some(self.skills.remove(index))
            } else {
                None
            }
        }

        pub fn get_active_skill(&self) -> Option<&String> {
            self.skills.get(self.active_skill_index)
        }

        pub fn set_active_skill(&mut self, index: usize) -> bool {
            if index < self.skills.len() {
                self.active_skill_index = index;
                true
            } else {
                false
            }
        }

        pub fn list_skills(&self) -> &[String] {
            &self.skills
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_skill_fan_in() {
            let mut skill_fan = SkillFanIn::new();
            skill_fan.add_skill("Fireball");
            skill_fan.add_skill("Ice Shard");

            assert_eq!(skill_fan.list_skills(), &[String::from("Fireball"), String::from("Ice Shard")]);
            assert_eq!(skill_fan.get_active_skill(), Some(&String::from("Fireball")));

            skill_fan.set_active_skill(1);
            assert_eq!(skill_fan.get_active_skill(), Some(&String::from("Ice Shard")));

            let removed_skill = skill_fan.remove_skill(0);
            assert_eq!(removed_skill, Some(String::from("Fireball")));
            assert_eq!(skill_fan.list_skills(), &[String::from("Ice Shard")]);
        }
    }
}
