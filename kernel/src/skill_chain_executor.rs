extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    let mut executor = SkillChainExecutor::new();
    executor.add_skill("Skill 1".into());
    executor.add_skill("Skill 2".into());
    executor.execute_skills();
    loop {}
}

pub struct SkillChainExecutor {
    skills: Vec<String>,
}

impl SkillChainExecutor {
    pub fn new() -> Self {
        SkillChainExecutor {
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

    pub fn get_skills(&self) -> &Vec<String> {
        &self.skills
    }

    pub fn execute_skills(&self) {
        for skill in &self.skills {
            // Simulate executing a skill
            println!("Executing: {}", skill);
        }
    }

    pub fn clear_skills(&mut self) {
        self.skills.clear();
    }
}
