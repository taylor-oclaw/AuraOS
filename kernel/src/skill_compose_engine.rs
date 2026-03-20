extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct SkillComposeEngine {
    skills: Vec<String>,
}

impl SkillComposeEngine {
    pub fn new() -> Self {
        SkillComposeEngine {
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

    pub fn clear_skills(&mut self) {
        self.skills.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skill_compose_engine() {
        let mut engine = SkillComposeEngine::new();

        assert_eq!(engine.list_skills(), Vec::<String>::new());

        engine.add_skill(String::from("Fireball"));
        engine.add_skill(String::from("Ice Shard"));

        assert_eq!(engine.get_skill(0), Some(&String::from("Fireball")));
        assert_eq!(engine.get_skill(1), Some(&String::from("Ice Shard")));

        let removed_skill = engine.remove_skill(0);
        assert_eq!(removed_skill, Some(String::from("Fireball")));
        assert_eq!(engine.get_skill(0), Some(&String::from("Ice Shard")));

        engine.clear_skills();
        assert_eq!(engine.list_skills(), Vec::<String>::new());
    }
}
