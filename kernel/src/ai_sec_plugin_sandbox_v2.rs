extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct AiSecPluginSandboxV2 {
    name: String,
    version: u32,
    active: bool,
    policies: Vec<String>,
    logs: Vec<String>,
}

impl AiSecPluginSandboxV2 {
    pub fn new(name: &str, version: u32) -> Self {
        AiSecPluginSandboxV2 {
            name: String::from(name),
            version,
            active: false,
            policies: Vec::new(),
            logs: Vec::new(),
        }
    }

    pub fn activate(&mut self) {
        self.active = true;
        self.log(String::from("info"));
    }

    pub fn deactivate(&mut self) {
        self.active = false;
        self.log(String::from("info"));
    }

    pub fn add_policy(&mut self, policy: &str) {
        self.policies.push(String::from(policy));
        self.log(String::from("info"));
    }

    pub fn remove_policy(&mut self, policy: &str) -> bool {
        if let Some(index) = self.policies.iter().position(|p| p == policy) {
            self.policies.remove(index);
            self.log(String::from("info"));
            true
        } else {
            false
        }
    }

    pub fn get_logs(&self) -> &Vec<String> {
        &self.logs
    }

    fn log(&mut self, message: String) {
        self.logs.push(message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_sec_plugin_sandbox_v2() {
        let mut sandbox = AiSecPluginSandboxV2::new("TestSandbox", 1);

        assert_eq!(sandbox.name, "TestSandbox");
        assert_eq!(sandbox.version, 1);
        assert!(!sandbox.active);

        sandbox.activate();
        assert!(sandbox.active);

        sandbox.add_policy("Policy1");
        assert_eq!(sandbox.policies.len(), 1);

        let removed = sandbox.remove_policy("Policy1");
        assert!(removed);
        assert_eq!(sandbox.policies.len(), 0);

        sandbox.deactivate();
        assert!(!sandbox.active);

        let logs = sandbox.get_logs();
        assert_eq!(logs.len(), 4); // Activate, add policy, remove policy, deactivate
    }
}
