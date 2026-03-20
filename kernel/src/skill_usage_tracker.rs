extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut tracker = SkillUsageTracker::new();
    
    tracker.record_usage("AI-123", "SkillA");
    tracker.record_usage("AI-456", "SkillB");
    tracker.record_usage("AI-123", "SkillC");

    if tracker.has_used_skill("AI-123", "SkillA") {
        println!("AI-123 has used SkillA");
    }

    let skills = tracker.get_skills_for_ai("AI-456");
    for skill in skills.iter() {
        println!("AI-456 has used {}", skill);
    }

    loop {}
}

pub struct SkillUsageTracker {
    records: Vec<(String, String)>,
}

impl SkillUsageTracker {
    pub fn new() -> Self {
        SkillUsageTracker {
            records: Vec::new(),
        }
    }

    pub fn record_usage(&mut self, ai_id: &str, skill_name: &str) {
        self.records.push((String::from(ai_id), String::from(skill_name)));
    }

    pub fn has_used_skill(&self, ai_id: &str, skill_name: &str) -> bool {
        for (id, skill) in self.records.iter() {
            if id == ai_id && skill == skill_name {
                return true;
            }
        }
        false
    }

    pub fn get_skills_for_ai(&self, ai_id: &str) -> Vec<String> {
        let mut skills = Vec::new();
        for (id, skill) in self.records.iter() {
            if id == ai_id && !skills.contains(skill) {
                skills.push(String::from(skill));
            }
        }
        skills
    }

    pub fn get_ais_using_skill(&self, skill_name: &str) -> Vec<String> {
        let mut ais = Vec::new();
        for (id, skill) in self.records.iter() {
            if skill == skill_name && !ais.contains(id) {
                ais.push(String::from(id));
            }
        }
        ais
    }

    pub fn total_skill_usages(&self) -> usize {
        self.records.len()
    }
}
