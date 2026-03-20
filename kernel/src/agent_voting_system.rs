extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut voting_system = AgentVotingSystem::new();
    voting_system.add_candidate("Alice");
    voting_system.add_candidate("Bob");
    voting_system.vote("Alice");
    voting_system.vote("Bob");
    voting_system.vote("Alice");
    println!("Votes for Alice: {}", voting_system.get_votes_for_candidate("Alice"));
    println!("Votes for Bob: {}", voting_system.get_votes_for_candidate("Bob"));
    println!("Winner: {}", voting_system.get_winner());
}

pub struct AgentVotingSystem {
    candidates: Vec<String>,
    votes: Vec<usize>,
}

impl AgentVotingSystem {
    pub fn new() -> Self {
        AgentVotingSystem {
            candidates: Vec::new(),
            votes: Vec::new(),
        }
    }

    pub fn add_candidate(&mut self, name: &str) {
        if !self.candidates.contains(&name.to_string()) {
            self.candidates.push(name.to_string());
            self.votes.push(0);
        }
    }

    pub fn vote(&mut self, candidate_name: &str) -> bool {
        if let Some(index) = self.candidates.iter().position(|c| c == candidate_name) {
            self.votes[index] += 1;
            true
        } else {
            false
        }
    }

    pub fn get_votes_for_candidate(&self, candidate_name: &str) -> usize {
        if let Some(index) = self.candidates.iter().position(|c| c == candidate_name) {
            self.votes[index]
        } else {
            0
        }
    }

    pub fn get_winner(&self) -> String {
        if self.candidates.is_empty() {
            return String::from("No candidates");
        }

        let mut max_votes = 0;
        let mut winner_index = 0;

        for (i, &votes) in self.votes.iter().enumerate() {
            if votes > max_votes {
                max_votes = votes;
                winner_index = i;
            }
        }

        self.candidates[winner_index].clone()
    }
}
