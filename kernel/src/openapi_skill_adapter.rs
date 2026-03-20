extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct OpenAPISkillAdapter {
    skills: Vec<String>,
    active_skill: Option<usize>,
}

impl OpenAPISkillAdapter {
    pub fn new() -> Self {
        OpenAPISkillAdapter {
            skills: Vec::new(),
            active_skill: None,
        }
    }

    pub fn add_skill(&mut self, skill_name: &str) {
        self.skills.push(String::from(skill_name));
    }

    pub fn remove_skill(&mut self, skill_index: usize) -> Option<String> {
        if skill_index < self.skills.len() {
            Some(self.skills.remove(skill_index))
        } else {
            None
        }
    }

    pub fn list_skills(&self) -> Vec<&str> {
        self.skills.iter().map(|s| s.as_str()).collect()
    }

    pub fn activate_skill(&mut self, skill_index: usize) -> bool {
        if skill_index < self.skills.len() {
            self.active_skill = Some(skill_index);
            true
        } else {
            false
        }
    }

    pub fn get_active_skill(&self) -> Option<&str> {
        self.active_skill.map(|index| &self.skills[index])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_openapi_skill_adapter() {
        let mut adapter = OpenAPISkillAdapter::new();
        assert_eq!(adapter.list_skills(), vec![]);

        adapter.add_skill("Skill1");
        adapter.add_skill("Skill2");
        assert_eq!(adapter.list_skills(), vec!["Skill1", "Skill2"]);

        assert!(adapter.activate_skill(0));
        assert_eq!(adapter.get_active_skill(), Some("Skill1"));

        let removed_skill = adapter.remove_skill(1);
        assert_eq!(removed_skill, Some(String::from("Skill2")));
        assert_eq!(adapter.list_skills(), vec!["Skill1"]);
    }
}
