extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct SkillTrustScorer {
    scores: Vec<(String, u32)>,
}

impl SkillTrustScorer {
    pub fn new() -> Self {
        SkillTrustScorer {
            scores: Vec::new(),
        }
    }

    pub fn add_skill(&mut self, skill_name: String, score: u32) {
        self.scores.push((skill_name, score));
    }

    pub fn get_score(&self, skill_name: &str) -> Option<u32> {
        for (name, score) in &self.scores {
            if name == skill_name {
                return Some(*score);
            }
        }
        None
    }

    pub fn update_score(&mut self, skill_name: String, new_score: u32) -> bool {
        for entry in &mut self.scores {
            if entry.0 == skill_name {
                entry.1 = new_score;
                return true;
            }
        }
        false
    }

    pub fn remove_skill(&mut self, skill_name: &str) -> bool {
        let pos = self.scores.iter().position(|(name, _)| name == skill_name);
        if let Some(index) = pos {
            self.scores.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_skills(&self) -> Vec<(String, u32)> {
        self.scores.clone()
    }
}
