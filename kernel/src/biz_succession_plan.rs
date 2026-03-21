extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct BizSuccessionPlan {
    name: String,
    candidates: Vec<String>,
    positions: Vec<String>,
    election_results: Vec<(String, usize)>, // (candidate_name, votes)
}

impl BizSuccessionPlan {
    pub fn new(name: &str) -> Self {
        BizSuccessionPlan {
            name: String::from(name),
            candidates: Vec::new(),
            positions: Vec::new(),
            election_results: Vec::new(),
        }
    }

    pub fn add_candidate(&mut self, candidate_name: &str) {
        self.candidates.push(String::from(candidate_name));
    }

    pub fn add_position(&mut self, position_name: &str) {
        self.positions.push(String::from(position_name));
    }

    pub fn conduct_election(&mut self, votes: Vec<(String, usize)>) -> bool {
        if votes.is_empty() || votes.len() != self.candidates.len() {
            return false;
        }
        self.election_results = votes;
        true
    }

    pub fn get_winner(&self) -> Option<&str> {
        self.election_results.iter()
            .max_by_key(|&(_, votes)| votes)
            .map(|&(ref candidate, _)| candidate.as_str())
    }

    pub fn get_election_results(&self) -> &Vec<(String, usize)> {
        &self.election_results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_biz_succession_plan() {
        let mut plan = BizSuccessionPlan::new("CEO Succession");
        plan.add_candidate("Alice");
        plan.add_candidate("Bob");
        plan.add_position("CEO");

        assert!(plan.conduct_election(vec![
            (String::from("Alice"), 10),
            (String::from("Bob"), 20),
        ];

        assert_eq!(plan.get_winner(), Some("Bob"));
        assert_eq!(plan.get_election_results().len(), 2);
    }
}
