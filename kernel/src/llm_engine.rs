extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub enum LlmTier { Tiny, Small, Medium, Large }

pub struct LlmModel {
    pub name: String,
    pub tier: LlmTier,
    pub size_mb: u64,
    pub loaded: bool,
    pub cost_per_token: u64,
    pub max_context: u32,
}

pub struct LlmRequest {
    pub id: u64,
    pub model_name: String,
    pub prompt: String,
    pub max_tokens: u32,
    pub agent_id: u64,
}

pub struct LlmResponse {
    pub request_id: u64,
    pub text: String,
    pub tokens_used: u32,
    pub cost: u64,
}

pub struct LlmEngine {
    pub models: Vec<LlmModel>,
    pub pending: Vec<LlmRequest>,
    pub completed: Vec<LlmResponse>,
    pub next_id: u64,
    pub total_tokens: u64,
    pub total_cost: u64,
}

impl LlmEngine {
    pub fn new() -> Self {
        Self {
            models: vec![
                LlmModel { name: String::from("aura-tiny"), tier: LlmTier::Tiny, size_mb: 500, loaded: false, cost_per_token: 1, max_context: 2048 },
                LlmModel { name: String::from("aura-small"), tier: LlmTier::Small, size_mb: 2000, loaded: false, cost_per_token: 5, max_context: 8192 },
                LlmModel { name: String::from("aura-medium"), tier: LlmTier::Medium, size_mb: 8000, loaded: false, cost_per_token: 20, max_context: 32768 },
            ],
            pending: Vec::new(),
            completed: Vec::new(),
            next_id: 1,
            total_tokens: 0,
            total_cost: 0,
        }
    }

    pub fn submit(&mut self, model: &str, prompt: &str, agent: u64) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.pending.push(LlmRequest {
            id, model_name: String::from(model), prompt: String::from(prompt),
            max_tokens: 1024, agent_id: agent,
        });
        id
    }

    pub fn cheapest_model(&self) -> Option<&str> {
        self.models.iter().min_by_key(|m| m.cost_per_token).map(|m| m.name.as_str())
    }

    pub fn load_model(&mut self, name: &str) -> bool {
        if let Some(m) = self.models.iter_mut().find(|m| m.name == name) {
            m.loaded = true; true
        } else { false }
    }

    pub fn budget_remaining(&self, budget: u64) -> u64 {
        if budget > self.total_cost { budget - self.total_cost } else { 0 }
    }

    pub fn pending_count(&self) -> usize { self.pending.len() }
}
