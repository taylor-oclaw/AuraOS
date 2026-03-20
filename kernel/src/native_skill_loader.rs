extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let loader = NativeSkillLoader::new();
    loader.load_skills();
    loop {}
}

pub struct NativeSkillLoader {
    skills: Vec<String>,
}

impl NativeSkillLoader {
    pub fn new() -> Self {
        NativeSkillLoader {
            skills: Vec::new(),
        }
    }

    pub fn add_skill(&mut self, skill_name: &str) {
        self.skills.push(String::from(skill_name));
    }

    pub fn remove_skill(&mut self, skill_name: &str) {
        if let Some(index) = self.skills.iter().position(|s| s == skill_name) {
            self.skills.remove(index);
        }
    }

    pub fn list_skills(&self) -> Vec<String> {
        self.skills.clone()
    }

    pub fn has_skill(&self, skill_name: &str) -> bool {
        self.skills.contains(&String::from(skill_name))
    }

    pub fn load_skills(&mut self) {
        // Simulate loading skills
        self.add_skill("Natural Language Processing");
        self.add_skill("Computer Vision");
        self.add_skill("Machine Learning");
        self.add_skill("Speech Recognition");
        self.add_skill("Reinforcement Learning");
    }
}
