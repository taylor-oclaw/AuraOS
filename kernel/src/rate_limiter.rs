extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct RateBucket {
    pub agent_id: u64,
    pub tokens: u32,
    pub max_tokens: u32,
    pub refill_rate: u32,
    pub last_refill: u64,
    pub total_requests: u64,
    pub rejected_requests: u64
}

pub struct RateLimiter {
    pub buckets: Vec<RateBucket>,
    pub global_limit: u32,
    pub global_used: u32
}

impl RateLimiter {
    pub fn new(global_limit: u32) -> Self {
        Self {
            buckets: Vec::new(),
            global_limit,
            global_used: 0
        }
    }

    pub fn register(&mut self, agent_id: u64, max_tokens: u32, refill_rate: u32) {
        self.buckets.push(RateBucket {
            agent_id,
            tokens: max_tokens,
            max_tokens,
            refill_rate,
            last_refill: 0,
            total_requests: 0,
            rejected_requests: 0
        });
    }

    pub fn try_acquire(&mut self, agent_id: u64, tokens: u32) -> bool {
        if self.global_used + tokens > self.global_limit {
            return false;
        }
        if let Some(b) = self.buckets.iter_mut().find(|b| b.agent_id == agent_id) {
            b.total_requests += 1;
            if b.tokens >= tokens {
                b.tokens -= tokens;
                self.global_used += tokens;
                true
            } else {
                b.rejected_requests += 1;
                false
            }
        } else {
            false
        }
    }

    pub fn refill(&mut self, agent_id: u64) {
        if let Some(b) = self.buckets.iter_mut().find(|b| b.agent_id == agent_id) {
            b.tokens = (b.tokens + b.refill_rate).min(b.max_tokens);
        }
    }

    pub fn refill_all(&mut self) {
        for b in &mut self.buckets {
            b.tokens = (b.tokens + b.refill_rate).min(b.max_tokens);
        }
        self.global_used = 0;
    }

    pub fn rejection_rate(&self, agent_id: u64) -> f32 {
        self.buckets.iter().find(|b| b.agent_id == agent_id).map(|b| if b.total_requests > 0 {
            b.rejected_requests as f32 / b.total_requests as f32
        } else {
            0.0
        }).unwrap_or(0.0)
    }
}
