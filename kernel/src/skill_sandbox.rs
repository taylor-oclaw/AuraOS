extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut sandbox = SkillSandbox::new();
    sandbox.add_skill("Programming");
    sandbox.add_skill("Machine Learning");
    sandbox.add_skill("Natural Language Processing");
    sandbox.add_skill("Computer Vision");
    sandbox.add_skill("Robotics");

    if sandbox.has_skill("Machine Learning") {
        println!("Skill found: Machine Learning");
    }

    let skills = sandbox.get_skills();
    for skill in skills.iter() {
        println!("Skill: {}", skill);
    }

    sandbox.remove_skill("Natural Language Processing");

    let remaining_skills = sandbox.get_skills();
    for skill in remaining_skills.iter() {
        println!("Remaining Skill: {}", skill);
    }

    loop {}
}

pub struct SkillSandbox {
    skills: Vec<String>,
}

impl SkillSandbox {
    pub fn new() -> Self {
        SkillSandbox {
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

    pub fn get_skills(&self) -> Vec<String> {
        self.skills.clone()
    }

    pub fn clear_skills(&mut self) {
        self.skills.clear();
    }
}
