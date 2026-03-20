extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct TrustEvent {
    pub action: String,
    pub success: bool,
    pub timestamp: u64,
    pub cost: u64,
}

pub struct AgentTrust {
    pub agent_id: u64,
    pub score: f32,
    pub events: Vec<TrustEvent>,
    pub sandbox_passes: u32,
    pub sandbox_fails: u32,
    pub tier: TrustTier,
    pub promoted_at: Option<u64>,
}

pub enum TrustTier {
    Untrusted,
    Sandboxed,
    Limited,
    Standard,
    Elevated,
    Autonomous,
}

impl AgentTrust {
    pub fn new(agent_id: u64) -> Self {
        Self {
            agent_id,
            score: 0.0,
            events: Vec::new(),
            sandbox_passes: 0,
            sandbox_fails: 0,
            tier: TrustTier::Untrusted,
            promoted_at: None,
        }
    }

    pub fn record_success(&mut self, action: &str, cost: u64) {
        self.events.push(TrustEvent {
            action: String::from(action),
            success: true,
            timestamp: 0,
            cost,
        });
        self.score = (self.score + 0.1).min(1.0);
        self.evaluate_tier();
    }

    pub fn record_failure(&mut self, action: &str) {
        self.events.push(TrustEvent {
            action: String::from(action),
            success: false,
            timestamp: 0,
            cost: 0,
        });
        self.score = (self.score - 0.3).max(0.0);
        self.evaluate_tier();
    }

    pub fn sandbox_pass(&mut self) {
        self.sandbox_passes += 1;
        self.score = (self.score + 0.15).min(1.0);
        self.evaluate_tier();
    }

    fn evaluate_tier(&mut self) {
        self.tier = if self.score >= 0.9 && self.sandbox_passes >= 10 {
            TrustTier::Autonomous
        } else if self.score >= 0.7 && self.sandbox_passes >= 5 {
            TrustTier::Elevated
        } else if self.score >= 0.5 && self.sandbox_passes >= 3 {
            TrustTier::Standard
        } else if self.score >= 0.3 {
            TrustTier::Limited
        } else if self.sandbox_passes > 0 {
            TrustTier::Sandboxed
        } else {
            TrustTier::Untrusted
        };
    }

    pub fn success_rate(&self) -> f32 {
        if self.events.is_empty() {
            0.0
        } else {
            self.events.iter().filter(|e| e.success).count() as f32 / self.events.len() as f32
        }
    }
}
