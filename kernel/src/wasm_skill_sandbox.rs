extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct WasmSkillSandbox {
    skills: Vec<String>,
    active_skill: Option<usize>,
}

impl WasmSkillSandbox {
    pub fn new() -> Self {
        WasmSkillSandbox {
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
    fn test_wasm_skill_sandbox() {
        let mut sandbox = WasmSkillSandbox::new();
        assert_eq!(sandbox.list_skills().len(), 0);

        sandbox.add_skill("AI Programming");
        sandbox.add_skill("Machine Learning");
        assert_eq!(sandbox.list_skills().len(), 2);
        assert_eq!(sandbox.list_skills()[0], "AI Programming");
        assert_eq!(sandbox.list_skills()[1], "Machine Learning");

        assert!(sandbox.activate_skill(0));
        assert_eq!(sandbox.get_active_skill(), Some("AI Programming"));

        assert_eq!(sandbox.remove_skill(1), Some(String::from("Machine Learning")));
        assert_eq!(sandbox.list_skills().len(), 1);

        assert!(!sandbox.activate_skill(1)); // Out of bounds
        assert_eq!(sandbox.get_active_skill(), Some("AI Programming"));
    }
}
