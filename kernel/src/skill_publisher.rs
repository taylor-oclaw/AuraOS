extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut skill_publisher = SkillPublisher::new();
    skill_publisher.add_skill("Rust Programming");
    skill_publisher.add_skill("Kernel Development");
    skill_publisher.add_skill("AI Algorithms");

    if skill_publisher.has_skill("Rust Programming") {
        println!("Skill found: Rust Programming");
    }

    let skills = skill_publisher.get_skills();
    for skill in skills.iter() {
        println!("Skill: {}", skill);
    }

    skill_publisher.remove_skill("Kernel Development");

    let remaining_skills = skill_publisher.get_skills();
    for skill in remaining_skills.iter() {
        println!("Remaining Skill: {}", skill);
    }

    loop {}
}

pub struct SkillPublisher {
    skills: Vec<String>,
}

impl SkillPublisher {
    pub fn new() -> Self {
        SkillPublisher {
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

    pub fn count_skills(&self) -> usize {
        self.skills.len()
    }
}
