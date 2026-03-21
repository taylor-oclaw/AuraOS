extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct BizStakeholderInfluence {
    stakeholders: Vec<String>,
    influence_levels: Vec<u32>,
}

impl BizStakeholderInfluence {
    pub fn new() -> Self {
        BizStakeholderInfluence {
            stakeholders: Vec::new(),
            influence_levels: Vec::new(),
        }
    }

    pub fn add_stakeholder(&mut self, name: &str, influence_level: u32) {
        self.stakeholders.push(String::from(name));
        self.influence_levels.push(influence_level);
    }

    pub fn get_stakeholder_count(&self) -> usize {
        self.stakeholders.len()
    }

    pub fn get_highest_influence(&self) -> Option<&str> {
        if self.stakeholders.is_empty() {
            return None;
        }
        let max_index = self.influence_levels.iter().enumerate().max_by_key(|&(_, &level)| level).map(|(index, _)| index);
        max_index.map(|index| &self.stakeholders[index])
    }

    pub fn get_average_influence(&self) -> Option<u32> {
        if self.influence_levels.is_empty() {
            return None;
        }
        let total: u32 = self.influence_levels.iter().sum();
        Some(total / self.influence_levels.len() as u32)
    }

    pub fn remove_stakeholder(&mut self, name: &str) -> bool {
        if let Some(index) = self.stakeholders.iter().position(|s| s == name) {
            self.stakeholders.remove(index);
            self.influence_levels.remove(index);
            true
        } else {
            false
        }
    }
}
