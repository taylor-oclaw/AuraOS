extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct SkillTimeoutHandler {
    timeouts: Vec<(String, u64)>,
}

impl SkillTimeoutHandler {
    pub fn new() -> Self {
        SkillTimeoutHandler {
            timeouts: Vec::new(),
        }
    }

    pub fn add_timeout(&mut self, skill_name: String, timeout: u64) {
        self.timeouts.push((skill_name, timeout));
    }

    pub fn remove_timeout(&mut self, skill_name: &str) -> bool {
        let pos = self.timeouts.iter().position(|(name, _)| name == skill_name);
        if let Some(index) = pos {
            self.timeouts.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_timeout(&self, skill_name: &str) -> Option<u64> {
        self.timeouts.iter().find_map(|(name, timeout)| {
            if name == skill_name {
                Some(*timeout)
            } else {
                None
            }
        }
    }

    pub fn has_expired(&self, current_time: u64) -> Vec<String> {
        self.timeouts
            .iter()
            .filter_map(|(name, timeout)| {
                if current_time > *timeout {
                    Some(name.clone())
                } else {
                    None
                }
            }
            .collect()
    }

    pub fn clear_expired(&mut self, current_time: u64) -> Vec<String> {
        let expired = self.has_expired(current_time);
        for skill_name in &expired {
            self.remove_timeout(skill_name);
        }
        expired
    }
))}
