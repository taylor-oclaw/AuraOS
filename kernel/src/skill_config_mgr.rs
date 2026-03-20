extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut config_mgr = SkillConfigMgr::new();
    config_mgr.add_skill("AI-101", "Beginner");
    config_mgr.add_skill("AI-202", "Intermediate");
    config_mgr.add_skill("AI-303", "Advanced");

    config_mgr.remove_skill("AI-101");
}

pub struct SkillConfigMgr {
    skills: Vec<(String, String)>,
}

impl SkillConfigMgr {
    pub fn new() -> Self {
        SkillConfigMgr { skills: Vec::new() }
    }

    pub fn add_skill(&mut self, skill_id: &str, level: &str) {
        self.skills.push((String::from(skill_id), String::from(level)));
    }

    pub fn remove_skill(&mut self, skill_id: &str) {
        self.skills.retain(|&(ref id, _)| id != skill_id);
    }

    pub fn get_skill_level(&self, skill_id: &str) -> Option<&String> {
        self.skills.iter().find_map(|(id, level)| if id == skill_id { Some(level) } else { None })
    }

    pub fn skill_count(&self) -> usize {
        self.skills.len()
    }

    pub fn list_skills(&self) -> Vec<(String, String)> {
        self.skills.clone()
    }
}
