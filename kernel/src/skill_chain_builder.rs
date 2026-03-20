extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut builder = SkillChainBuilder::new();
    builder.add_skill("Programming");
    builder.add_skill("Machine Learning");
    builder.add_skill("Natural Language Processing");
    builder.add_skill("Computer Vision");
    builder.add_skill("Robotics");

    if builder.has_skill("Machine Learning") {
        println!("Skill found: Machine Learning");
    }

    let skills = builder.get_skills();
    for skill in skills.iter() {
        println!("Skill: {}", skill);
    }

    loop {}
}

pub struct SkillChainBuilder {
    skills: Vec<String>,
}

impl SkillChainBuilder {
    pub fn new() -> Self {
        SkillChainBuilder {
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
