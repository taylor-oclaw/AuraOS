extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct SkillFanOut {
    skills: Vec<String>,
}

impl SkillFanOut {
    pub fn new() -> Self {
        SkillFanOut {
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

    pub fn get_skill(&self, index: usize) -> Option<&String> {
        self.skills.get(index)
    }

    pub fn list_skills(&self) -> &[String] {
        &self.skills
    }

    pub fn clear_skills(&mut self) {
        self.skills.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skill_fan_out() {
        let mut skill_fan_out = SkillFanOut::new();

        assert_eq!(skill_fan_out.list_skills().len(), 0);

        skill_fan_out.add_skill(String::from("Rust"));
        skill_fan_out.add_skill(String::from("AI"));

        assert_eq!(skill_fan_out.list_skills().len(), 2);
        assert_eq!(skill_fan_out.get_skill(0), Some(&String::from("Rust")));
        assert_eq!(skill_fan_out.get_skill(1), Some(&String::from("AI")));

        let removed_skill = skill_fan_out.remove_skill(0);
        assert_eq!(removed_skill, Some(String::from("Rust")));
        assert_eq!(skill_fan_out.list_skills().len(), 1);

        skill_fan_out.clear_skills();
        assert_eq!(skill_fan_out.list_skills().len(), 0);
    }
}
