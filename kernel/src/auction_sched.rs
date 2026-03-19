extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub type AgentId = u64;

pub struct Bid {
    pub agent_id: AgentId,
    pub requested_ms: u64,
    pub urgency: f32,
    pub max_cost: u64,
    pub description: String,
}

pub struct ScheduleSlot {
    pub agent_id: AgentId,
    pub time_ms: u64,
    pub actual_cost: u64,
}

pub struct AuctionScheduler {
    pub pending_bids: Vec<Bid>,
    pub schedule: Vec<ScheduleSlot>,
    pub total_compute_budget_ms: u64,
    pub used_ms: u64,
    pub round: u64,
}

impl AuctionScheduler {
    pub fn new(budget_ms: u64) -> Self {
        Self {
            pending_bids: Vec::new(),
            schedule: Vec::new(),
            total_compute_budget_ms: budget_ms,
            used_ms: 0,
            round: 0,
        }
    }

    pub fn submit_bid(&mut self, bid: Bid) {
        self.pending_bids.push(bid);
    }

    pub fn run_auction(&mut self) -> Vec<ScheduleSlot> {
        self.round += 1;
        self.pending_bids.sort_by(|a, b| b.urgency.partial_cmp(&a.urgency).unwrap_or(core::cmp::Ordering::Equal));
        let mut result = Vec::new();
        let mut remaining = self.total_compute_budget_ms - self.used_ms;

        for bid in self.pending_bids.drain(..) {
            if bid.requested_ms <= remaining {
                let slot = ScheduleSlot {
                    agent_id: bid.agent_id,
                    time_ms: bid.requested_ms,
                    actual_cost: (bid.requested_ms as f32 * bid.urgency) as u64,
                };
                remaining -= bid.requested_ms;
                self.used_ms += bid.requested_ms;
                result.push(slot);
            }
        }

        self.schedule.extend(result.iter().map(|s| ScheduleSlot {
            agent_id: s.agent_id,
            time_ms: s.time_ms,
            actual_cost: s.actual_cost,
        }));

        result
    }

    pub fn reset_budget(&mut self) {
        self.used_ms = 0;
    }

    pub fn utilization(&self) -> f32 {
        self.used_ms as f32 / self.total_compute_budget_ms as f32
    }
}
