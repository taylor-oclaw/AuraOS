extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct AgentCircuitBreaker {
    state: CircuitState,
    failure_threshold: u32,
    success_threshold: u32,
    consecutive_failures: u32,
    consecutive_successes: u32,
}

enum CircuitState {
    Closed,
    Open,
    HalfOpen,
}

impl AgentCircuitBreaker {
    pub fn new(failure_threshold: u32, success_threshold: u32) -> Self {
        AgentCircuitBreaker {
            state: CircuitState::Closed,
            failure_threshold,
            success_threshold,
            consecutive_failures: 0,
            consecutive_successes: 0,
        }
    }

    pub fn record_failure(&mut self) {
        if let CircuitState::Closed = self.state {
            self.consecutive_failures += 1;
            if self.consecutive_failures >= self.failure_threshold {
                self.state = CircuitState::Open;
                self.consecutive_successes = 0;
            }
        } else if let CircuitState::HalfOpen = self.state {
            self.state = CircuitState::Open;
            self.consecutive_successes = 0;
        }
    }

    pub fn record_success(&mut self) {
        match self.state {
            CircuitState::Closed => {
                self.consecutive_successes += 1;
                if self.consecutive_successes >= self.success_threshold {
                    self.consecutive_failures = 0;
                }
            }
            CircuitState::HalfOpen => {
                self.consecutive_successes += 1;
                if self.consecutive_successes >= self.success_threshold {
                    self.state = CircuitState::Closed;
                    self.consecutive_failures = 0;
                } else {
                    self.state = CircuitState::Open;
                }
            }
            _ => {}
        }
    }

    pub fn is_allowed(&self) -> bool {
        matches!(self.state, CircuitState::Closed | CircuitState::HalfOpen)
    }

    pub fn transition_to_half_open(&mut self) {
        if let CircuitState::Open = self.state {
            self.state = CircuitState::HalfOpen;
            self.consecutive_successes = 0;
        }
    }

    pub fn get_state(&self) -> &str {
        match self.state {
            CircuitState::Closed => "Closed",
            CircuitState::Open => "Open",
            CircuitState::HalfOpen => "HalfOpen",
        }
    }
}
