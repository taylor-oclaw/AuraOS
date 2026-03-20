extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_std]
mod agent_compliance_engine {
    use core::fmt::{Debug, Formatter};
    use alloc::boxed::Box;

    pub struct ComplianceEngine {
        policies: Vec<String>,
        violations: Vec<String>,
    }

    impl Debug for ComplianceEngine {
        fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
            write!(f, "ComplianceEngine {{ policies: {:?}, violations: {:?} }}", self.policies, self.violations)
        }
    }

    impl ComplianceEngine {
        pub fn new() -> Self {
            ComplianceEngine {
                policies: Vec::new(),
                violations: Vec::new(),
            }
        }

        pub fn add_policy(&mut self, policy: String) {
            self.policies.push(policy);
        }

        pub fn check_compliance(&self, action: &str) -> bool {
            !self.policies.iter().any(|policy| action.contains(policy))
        }

        pub fn log_violation(&mut self, violation: String) {
            self.violations.push(violation);
        }

        pub fn get_policies(&self) -> &[String] {
            &self.policies
        }

        pub fn get_violations(&self) -> &[String] {
            &self.violations
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_compliance_engine() {
            let mut engine = ComplianceEngine::new();
            engine.add_policy(String::from("no_root_access"));
            assert!(engine.check_compliance("user_login"));
            assert!(!engine.check_compliance("root_login"));
            engine.log_violation(String::from("Attempted root access"));
            assert_eq!(engine.get_policies().len(), 1);
            assert_eq!(engine.get_violations().len(), 1);
        }
    }
}
