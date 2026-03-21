extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct PriorityStakeholderWeight {
    stakeholders: Vec<(String, u32)>,
}

impl PriorityStakeholderWeight {
    pub fn new() -> Self {
        PriorityStakeholderWeight {
            stakeholders: Vec::new(),
        }
    }

    pub fn add_stakeholder(&mut self, name: String, weight: u32) {
        self.stakeholders.push((name, weight));
    }

    pub fn remove_stakeholder(&mut self, name: &str) -> bool {
        let pos = self.stakeholders.iter().position(|(n, _)| n == name);
        if let Some(index) = pos {
            self.stakeholders.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_weight(&self, name: &str) -> Option<u32> {
        self.stakeholders.iter().find(|(n, _)| n == name).map(|(_, w)| *w)
    }

    pub fn total_weight(&self) -> u32 {
        self.stakeholders.iter().map(|(_, w)| w).sum()
    }

    pub fn top_stakeholder(&self) -> Option<&String> {
        self.stakeholders
            .iter()
            .max_by_key(|&(_, weight)| weight)
            .map(|(name, _)| name)
    }
}
