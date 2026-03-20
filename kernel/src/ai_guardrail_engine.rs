extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let engine = AIGuardrailEngine::new();
    engine.initialize();
    loop {}
}

pub struct AIGuardrailEngine {
    policies: Vec<String>,
    logs: Vec<String>,
}

impl AIGuardrailEngine {
    pub fn new() -> Self {
        AIGuardrailEngine {
            policies: Vec::new(),
            logs: Vec::new(),
        }
    }

    pub fn add_policy(&mut self, policy: String) {
        self.policies.push(policy);
    }

    pub fn remove_policy(&mut self, index: usize) -> Option<String> {
        if index < self.policies.len() {
            Some(self.policies.remove(index))
        } else {
            None
        }
    }

    pub fn list_policies(&self) -> Vec<String> {
        self.policies.clone()
    }

    pub fn log_event(&mut self, event: String) {
        self.logs.push(event);
    }

    pub fn get_logs(&self) -> Vec<String> {
        self.logs.clone()
    }
}
