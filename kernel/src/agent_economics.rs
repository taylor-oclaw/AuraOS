extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AgentBudget {
    pub agent_id: u64,
    pub daily_limit: u64,
    pub spent_today: u64,
    pub total_spent: u64,
    pub earnings: u64,
    pub transactions: Vec<Transaction>,
}

pub struct Transaction {
    pub amount: u64,
    pub description: String,
    pub agent_id: u64,
    pub timestamp: u64,
    pub is_debit: bool,
}

pub struct AgentEconomics {
    pub budgets: Vec<AgentBudget>,
    pub total_budget: u64,
    pub total_spent: u64,
    pub cost_per_llm_call: u64,
    pub cost_per_tool_use: u64,
}

impl AgentEconomics {
    pub fn new(total: u64) -> Self {
        Self {
            budgets: Vec::new(),
            total_budget: total,
            total_spent: 0,
            cost_per_llm_call: 10,
            cost_per_tool_use: 5,
        }
    }

    pub fn create_budget(&mut self, agent_id: u64, daily_limit: u64) {
        self.budgets.push(AgentBudget {
            agent_id,
            daily_limit,
            spent_today: 0,
            total_spent: 0,
            earnings: 0,
            transactions: Vec::new(),
        };
    }

    pub fn charge(&mut self, agent_id: u64, amount: u64, desc: &str) -> bool {
        if let Some(b) = self.budgets.iter_mut().find(|b| b.agent_id == agent_id) {
            if b.spent_today + amount <= b.daily_limit {
                b.spent_today += amount;
                b.total_spent += amount;
                self.total_spent += amount;
                b.transactions.push(Transaction {
                    amount,
                    description: String::from(desc),
                    agent_id,
                    timestamp: 0,
                    is_debit: true,
                };
                return true;
            }
        }
        false
    }

    pub fn credit(&mut self, agent_id: u64, amount: u64, desc: &str) {
        if let Some(b) = self.budgets.iter_mut().find(|b| b.agent_id == agent_id) {
            b.earnings += amount;
            b.transactions.push(Transaction {
                amount,
                description: String::from(desc),
                agent_id,
                timestamp: 0,
                is_debit: false,
            };
        }
    }

    pub fn reset_daily(&mut self) {
        for b in &mut self.budgets {
            b.spent_today = 0;
        }
    }

    pub fn over_budget(&self) -> Vec<u64> {
        self.budgets.iter().filter(|b| b.spent_today >= b.daily_limit).map(|b| b.agent_id).collect()
    }

    pub fn remaining(&self) -> u64 {
        self.total_budget.saturating_sub(self.total_spent)
    }
)))}
