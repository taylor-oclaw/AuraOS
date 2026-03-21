extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut engine = EnterpriseDlpEngine::new();
    engine.initialize();
    loop {}
}

pub struct EnterpriseDlpEngine {
    policies: Vec<String>,
    logs: Vec<String>,
    active: bool,
}

impl EnterpriseDlpEngine {
    pub fn new() -> Self {
        EnterpriseDlpEngine {
            policies: Vec::new(),
            logs: Vec::new(),
            active: false,
        }
    }

    pub fn initialize(&mut self) {
        // Load default policies
        self.policies.push(String::from("No confidential data sharing"));
        self.active = true;
        self.log_event(String::from("Engine initialized and activated"));
    }

    pub fn add_policy(&mut self, policy: String) {
        if !self.policies.contains(&policy) {
            self.policies.push(policy);
            self.log_event(format!("Policy added: {}", &self.policies.last().unwrap()));
        }
    }

    pub fn remove_policy(&mut self, policy: &str) -> bool {
        let index = self.policies.iter().position(|p| p == policy);
        if let Some(i) = index {
            let removed_policy = self.policies.remove(i);
            self.log_event(format!("Policy removed: {}", removed_policy));
            true
        } else {
            false
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
