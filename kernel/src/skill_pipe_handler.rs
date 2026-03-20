extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct SkillPipeHandler {
    skills: Vec<String>,
}

impl SkillPipeHandler {
    pub fn new() -> Self {
        SkillPipeHandler {
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

    pub fn list_skills(&self) -> Vec<String> {
        self.skills.clone()
    }

    pub fn skill_count(&self) -> usize {
        self.skills.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skill_pipe_handler() {
        let mut handler = SkillPipeHandler::new();
        assert_eq!(handler.skill_count(), 0);

        handler.add_skill(String::from("AI"));
        handler.add_skill(String::from("Machine Learning"));
        assert_eq!(handler.skill_count(), 2);

        assert_eq!(handler.get_skill(0), Some(&String::from("AI")));
        assert_eq!(handler.get_skill(1), Some(&String::from("Machine Learning")));

        let removed = handler.remove_skill(0);
        assert_eq!(removed, Some(String::from("AI")));
        assert_eq!(handler.skill_count(), 1);

        let skills = handler.list_skills();
        assert_eq!(skills, vec![String::from("Machine Learning")]);
    }
}
