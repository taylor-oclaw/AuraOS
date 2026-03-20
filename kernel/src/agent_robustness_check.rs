extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct AgentRobustnessCheck {
    checks_passed: usize,
    checks_failed: usize,
    results: Vec<String>,
}

impl AgentRobustnessCheck {
    pub fn new() -> Self {
        AgentRobustnessCheck {
            checks_passed: 0,
            checks_failed: 0,
            results: Vec::new(),
        }
    }

    pub fn run_check(&mut self, check_name: &str, condition: bool) {
        if condition {
            self.checks_passed += 1;
            self.results.push(format!("{} passed", check_name));
        } else {
            self.checks_failed += 1;
            self.results.push(format!("{} failed", check_name));
        }
    }

    pub fn get_checks_passed(&self) -> usize {
        self.checks_passed
    }

    pub fn get_checks_failed(&self) -> usize {
        self.checks_failed
    }

    pub fn get_results(&self) -> &Vec<String> {
        &self.results
    }

    pub fn reset(&mut self) {
        self.checks_passed = 0;
        self.checks_failed = 0;
        self.results.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_robustness_check() {
        let mut check = AgentRobustnessCheck::new();

        check.run_check("Check 1", true);
        check.run_check("Check 2", false);
        check.run_check("Check 3", true);

        assert_eq!(check.get_checks_passed(), 2);
        assert_eq!(check.get_checks_failed(), 1);

        let results = check.get_results();
        assert_eq!(results.len(), 3);
        assert_eq!(results[0], "Check 1 passed");
        assert_eq!(results[1], "Check 2 failed");
        assert_eq!(results[2], "Check 3 passed");

        check.reset();
        assert_eq!(check.get_checks_passed(), 0);
        assert_eq!(check.get_checks_failed(), 0);
        assert!(check.get_results().is_empty());
    }
}
