extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

mod ai_guardrail_chain {
    use super::*;

    pub struct AIGuardrailChain {
        rules: Vec<String>,
        violations: Vec<String>,
    }

    impl AIGuardrailChain {
        pub fn new() -> Self {
            AIGuardrailChain {
                rules: Vec::new(),
                violations: Vec::new(),
            }
        }

        pub fn add_rule(&mut self, rule: String) {
            self.rules.push(rule);
        }

        pub fn remove_rule(&mut self, index: usize) -> Option<String> {
            if index < self.rules.len() {
                Some(self.rules.remove(index))
            } else {
                None
            }
        }

        pub fn check_violation(&self, action: &str) -> bool {
            for rule in &self.rules {
                if action.contains(rule) {
                    self.violations.push(action.to_string());
                    return true;
                }
            }
            false
        }

        pub fn get_rules(&self) -> &[String] {
            &self.rules
        }

        pub fn get_violations(&self) -> &[String] {
            &self.violations
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ai_guardrail_chain::*;

    #[test]
    fn test_ai_guardrail_chain() {
        let mut chain = AIGuardrailChain::new();
        chain.add_rule(String::from("no_hacking"));
        chain.add_rule(String::from("no_spam"));

        assert_eq!(chain.get_rules().len(), 2);
        assert!(!chain.check_violation("user is writing a blog post"));
        assert!(chain.check_violation("user is hacking the system"));
        assert_eq!(chain.get_violations().len(), 1);

        chain.remove_rule(0);
        assert_eq!(chain.get_rules().len(), 1);
    }
}
