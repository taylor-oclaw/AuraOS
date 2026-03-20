extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct SkillConditional {
    skills: Vec<String>,
    conditions: Vec<String>,
}

impl SkillConditional {
    pub fn new(skills: Vec<String>, conditions: Vec<String>) -> Self {
        SkillConditional { skills, conditions }
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

    pub fn add_condition(&mut self, condition: String) {
        self.conditions.push(condition);
    }

    pub fn remove_condition(&mut self, condition: &str) -> bool {
        if let Some(index) = self.conditions.iter().position(|c| c == condition) {
            self.conditions.remove(index);
            true
        } else {
            false
        }
    }

    pub fn check_conditions(&self, skill: &str) -> bool {
        self.skills.contains(&skill.to_string()) && self.conditions.iter().all(|c| c == "met")
    }
}
