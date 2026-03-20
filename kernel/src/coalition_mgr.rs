extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct Coalition {
    pub id: u64,
    pub name: String,
    pub members: Vec<u64>,
    pub leader: u64,
    pub goal: String,
    pub formed_at: u64,
    pub active: bool,
    pub total_value: u64,
}

pub struct CoalitionManager {
    pub coalitions: Vec<Coalition>,
    pub next_id: u64,
}

impl CoalitionManager {
    pub fn new() -> Self {
        Self {
            coalitions: Vec::new(),
            next_id: 1,
        }
    }

    pub fn form(&mut self, name: &str, leader: u64, goal: &str, members: Vec<u64>) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        let mut all_members = members;
        if !all_members.contains(&leader) {
            all_members.push(leader);
        }
        self.coalitions.push(Coalition {
            id,
            name: String::from(name),
            members: all_members,
            leader,
            goal: String::from(goal),
            formed_at: 0,
            active: true,
            total_value: 0,
        });
        id
    }

    pub fn join(&mut self, coalition_id: u64, agent_id: u64) -> bool {
        if let Some(c) = self.coalitions.iter_mut().find(|c| c.id == coalition_id && c.active) {
            if !c.members.contains(&agent_id) {
                c.members.push(agent_id);
                return true;
            }
        }
        false
    }

    pub fn leave(&mut self, coalition_id: u64, agent_id: u64) {
        if let Some(c) = self.coalitions.iter_mut().find(|c| c.id == coalition_id) {
            c.members.retain(|m| *m != agent_id);
            if c.members.is_empty() {
                c.active = false;
            }
        }
    }

    pub fn dissolve(&mut self, coalition_id: u64) {
        if let Some(c) = self.coalitions.iter_mut().find(|c| c.id == coalition_id) {
            c.active = false;
        }
    }

    pub fn agent_coalitions(&self, agent_id: u64) -> Vec<&Coalition> {
        self.coalitions
            .iter()
            .filter(|c| c.active && c.members.contains(&agent_id))
            .collect()
    }

    pub fn active_count(&self) -> usize {
        self.coalitions.iter().filter(|c| c.active).count()
    }

    pub fn total(&self) -> usize {
        self.coalitions.len()
    }
}
