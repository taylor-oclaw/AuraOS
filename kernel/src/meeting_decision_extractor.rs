extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct MeetingDecisionExtractor {
    decisions: Vec<String>,
}

impl MeetingDecisionExtractor {
    pub fn new() -> Self {
        MeetingDecisionExtractor {
            decisions: Vec::new(),
        }
    }

    pub fn add_decision(&mut self, decision: String) {
        self.decisions.push(decision);
    }

    pub fn get_decisions(&self) -> &Vec<String> {
        &self.decisions
    }

    pub fn remove_decision(&mut self, index: usize) -> Option<String> {
        if index < self.decisions.len() {
            Some(self.decisions.remove(index))
        } else {
            None
        }
    }

    pub fn clear_decisions(&mut self) {
        self.decisions.clear();
    }

    pub fn find_decision(&self, keyword: &str) -> Option<&String> {
        self.decisions.iter().find(|decision| decision.contains(keyword))
    }
}
