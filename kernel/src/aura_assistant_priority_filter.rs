extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AuraAssistantPriorityFilter {
    priority_levels: Vec<String>,
}

impl AuraAssistantPriorityFilter {
    pub fn new() -> Self {
        AuraAssistantPriorityFilter {
            priority_levels: Vec::new(),
        }
    }

    pub fn add_priority_level(&mut self, level: String) {
        if !self.priority_levels.contains(&level) {
            self.priority_levels.push(level);
        }
    }

    pub fn remove_priority_level(&mut self, level: &str) -> bool {
        let pos = self.priority_levels.iter().position(|x| x == level);
        if let Some(index) = pos {
            self.priority_levels.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_priority_levels(&self) -> Vec<String> {
        self.priority_levels.clone()
    }

    pub fn has_priority_level(&self, level: &str) -> bool {
        self.priority_levels.contains(&String::from(level))
    }

    pub fn clear_priority_levels(&mut self) {
        self.priority_levels.clear();
    }
}
