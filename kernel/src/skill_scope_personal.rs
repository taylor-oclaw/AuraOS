extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut skill_scope = SkillScopePersonal::new();
    skill_scope.add_skill("Rust Programming");
    skill_scope.add_skill("Kernel Development");
    skill_scope.add_skill("AI Algorithms");

    if skill_scope.has_skill("Rust Programming") {
    }

    let skills = skill_scope.list_skills();
    for skill in skills.iter() {
    }

    skill_scope.remove_skill("Kernel Development");
    let remaining_skills = skill_scope.list_skills();
    for skill in remaining_skills.iter() {
    }

    loop {}
}

pub struct SkillScopePersonal {
    skills: Vec<String>,
}

impl SkillScopePersonal {
    pub fn new() -> Self {
        SkillScopePersonal {
            skills: Vec::new(),
        }
    }

    pub fn add_skill(&mut self, skill: &str) {
        if !self.skills.contains(&String::from(skill)) {
            self.skills.push(String::from(skill));
        }
    }

    pub fn remove_skill(&mut self, skill: &str) {
        self.skills.retain(|s| s != skill);
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
