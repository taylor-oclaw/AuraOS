extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct AgentCitationEngine {
    citations: Vec<String>,
}

impl AgentCitationEngine {
    pub fn new() -> Self {
        AgentCitationEngine {
            citations: Vec::new(),
        }
    }

    pub fn add_citation(&mut self, citation: String) {
        self.citations.push(citation);
    }

    pub fn remove_citation(&mut self, index: usize) -> Option<String> {
        if index < self.citations.len() {
            Some(self.citations.remove(index))
        } else {
            None
        }
    }

    pub fn get_citation(&self, index: usize) -> Option<&String> {
        self.citations.get(index)
    }

    pub fn list_citations(&self) -> &[String] {
        &self.citations
    }

    pub fn count_citations(&self) -> usize {
        self.citations.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_citation_engine() {
        let mut engine = AgentCitationEngine::new();
        assert_eq!(engine.count_citations(), 0);

        engine.add_citation(String::from("Citation 1"));
        engine.add_citation(String::from("Citation 2"));
        assert_eq!(engine.count_citations(), 2);

        assert_eq!(engine.get_citation(0), Some(&String::from("Citation 1")));
        assert_eq!(engine.get_citation(1), Some(&String::from("Citation 2")));

        let removed = engine.remove_citation(0);
        assert_eq!(removed, Some(String::from("Citation 1")));
        assert_eq!(engine.count_citations(), 1);

        assert_eq!(engine.list_citations(), &[String::from("Citation 2")]);
    }
}
