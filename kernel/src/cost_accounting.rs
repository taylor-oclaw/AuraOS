extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct CostEntry {
    pub agent_id: u64,
    pub category: String,
    pub amount: u64,
    pub timestamp: u64,
    pub description: String,
}

pub struct AgentBudget {
    pub agent_id: u64,
    pub daily_limit: u64,
    pub spent_today: u64,
    pub total_spent: u64,
    pub warnings_sent: u32,
}

pub struct CostAccounting {
    pub ledger: Vec<CostEntry>,
    pub budgets: Vec<AgentBudget>,
    pub global_daily_budget: u64,
    pub global_spent_today: u64,
}

impl CostAccounting {
    pub fn new(daily_budget: u64) -> Self {
        Self {
            ledger: Vec::new(),
            budgets: Vec::new(),
            global_daily_budget: daily_budget,
            global_spent_today: 0,
        }
    }

    pub fn set_agent_budget(&mut self, agent_id: u64, daily: u64) {
        if let Some(b) = self.budgets.iter_mut().find(|b| b.agent_id == agent_id) {
            b.daily_limit = daily;
        } else {
            self.budgets.push(AgentBudget {
                agent_id,
                daily_limit: daily,
                spent_today: 0,
                total_spent: 0,
                warnings_sent: 0,
            };
        }
    }

    pub fn charge(&mut self, agent_id: u64, category: &str, amount: u64, desc: &str) -> bool {
        if let Some(b) = self.budgets.iter_mut().find(|b| b.agent_id == agent_id) {
            if b.spent_today + amount > b.daily_limit {
                return false;
            }
            b.spent_today += amount;
            b.total_spent += amount;
        }
        self.global_spent_today += amount;
        self.ledger.push(CostEntry {
            agent_id,
            category: String::from(category),
            amount,
            timestamp: 0,
            description: String::from(desc),
        };
        true
    }

    pub fn agent_spent(&self, agent_id: u64) -> u64 {
        self.budgets.iter().find(|b| b.agent_id == agent_id).map(|b| b.spent_today).unwrap_or(0)
    }

    pub fn reset_daily(&mut self) {
        for b in &mut self.budgets {
            b.spent_today = 0;
        }
        self.global_spent_today = 0;
    }

    pub fn over_budget(&self) -> Vec<u64> {
        self.budgets.iter().filter(|b| b.spent_today > b.daily_limit * 80 / 100).map(|b| b.agent_id).collect()
    }
))}
