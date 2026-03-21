extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub enum Vote {
    Approve,
    Reject,
    Abstain,
}

pub struct Proposal {
    pub id: u64,
    pub topic: String,
    pub proposed_by: u64,
    pub votes: Vec<(u64, Vote)>,
    pub quorum: usize,
    pub deadline: u64,
    pub resolved: bool,
    pub approved: Option<bool>,
}

pub struct ConsensusEngine {
    pub proposals: Vec<Proposal>,
    pub next_id: u64,
    pub default_quorum: usize,
}

impl ConsensusEngine {
    pub fn new(quorum: usize) -> Self {
        Self {
            proposals: Vec::new(),
            next_id: 1,
            default_quorum: quorum,
        }
    }

    pub fn propose(&mut self, agent_id: u64, topic: &str, deadline: u64) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.proposals.push(Proposal {
            id,
            topic: String::from(topic),
            proposed_by: agent_id,
            votes: Vec::new(),
            quorum: self.default_quorum,
            deadline,
            resolved: false,
            approved: None,
        };
        id
    }

    pub fn vote(&mut self, proposal_id: u64, agent_id: u64, vote: Vote) -> bool {
        if let Some(p) = self.proposals.iter_mut().find(|p| p.id == proposal_id && !p.resolved) {
            if !p.votes.iter().any(|(a, _)| *a == agent_id) {
                p.votes.push((agent_id, vote));
                return true;
            }
        }
        false
    }

    pub fn resolve(&mut self, proposal_id: u64) -> Option<bool> {
        if let Some(p) = self.proposals.iter_mut().find(|p| p.id == proposal_id && !p.resolved) {
            if p.votes.len() >= p.quorum {
                let approvals = p.votes.iter().filter(|(_, v)| matches!(v, Vote::Approve)).count();
                let rejections = p.votes.iter().filter(|(_, v)| matches!(v, Vote::Reject)).count();
                let approved = approvals > rejections;
                p.approved = Some(approved);
                p.resolved = true;
                return Some(approved);
            }
        }
        None
    }

    pub fn pending_proposals(&self) -> Vec<&Proposal> {
        self.proposals.iter().filter(|p| !p.resolved).collect()
    }

    pub fn total(&self) -> usize {
        self.proposals.len()
    }
)}
