extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut planner = SkillChainPlanner::new();
    planner.add_skill("Programming");
    planner.add_skill("Machine Learning");
    planner.add_skill("Data Analysis");
    planner.add_skill("Natural Language Processing");
    planner.add_skill("Computer Vision");

    if planner.has_skill("Machine Learning") {
    }

    let skills = planner.get_skills();
    for skill in skills.iter() {
    }

    loop {}
}

pub struct SkillChainPlanner {
    skills: Vec<String>,
}

impl SkillChainPlanner {
    pub fn new() -> Self {
        SkillChainPlanner {
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
