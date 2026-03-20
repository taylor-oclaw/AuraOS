extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut skill_scope = SkillScopeOrg::new();
    skill_scope.add_skill("Rust Programming");
    skill_scope.add_skill("Kernel Development");
    skill_scope.add_skill("AI Algorithms");

    if skill_scope.has_skill("Rust Programming") {
        println!("Skill found: Rust Programming");
    }

    let skills = skill_scope.list_skills();
    for skill in skills.iter() {
        println!("Skill: {}", skill);
    }

    loop {}
}

pub struct SkillScopeOrg {
    skills: Vec<String>,
}

impl SkillScopeOrg {
    pub fn new() -> Self {
        SkillScopeOrg {
            skills: Vec::new(),
        }
    }

    pub fn add_skill(&mut self, skill: &str) {
        self.skills.push(String::from(skill));
    }

    pub fn remove_skill(&mut self, skill: &str) {
        if let Some(index) = self.skills.iter().position(|s| s == skill) {
            self.skills.remove(index);
        }
    }

    pub fn has_skill(&self, skill: &str) -> bool {
        self.skills.contains(&String::from(skill))
    }

    pub fn list_skills(&self) -> Vec<String> {
        self.skills.clone()
    }

    pub fn count_skills(&self) -> usize {
        self.skills.len()
    }
}
