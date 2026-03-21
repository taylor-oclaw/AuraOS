extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub enum TestResult {
    Pass,
    Fail(String),
    Timeout,
    Skipped,
}

pub struct SandboxTest {
    pub name: String,
    pub category: String,
    pub result: TestResult,
    pub duration_ms: u64,
}

pub struct SandboxEval {
    pub agent_id: u64,
    pub tests: Vec<SandboxTest>,
    pub score: f32,
    pub passed: u32,
    pub failed: u32,
    pub total_time_ms: u64,
    pub promoted: bool,
}

impl SandboxEval {
    pub fn new(agent_id: u64) -> Self {
        Self {
            agent_id,
            tests: Vec::new(),
            score: 0.0,
            passed: 0,
            failed: 0,
            total_time_ms: 0,
            promoted: false,
        }
    }

    pub fn add_result(&mut self, name: &str, category: &str, result: TestResult, duration_ms: u64) {
        match &result {
            TestResult::Pass => self.passed += 1,
            TestResult::Fail(_) => self.failed += 1,
            _ => {}
        }
        self.total_time_ms += duration_ms;
        self.tests.push(SandboxTest {
            name: String::from(name),
            category: String::from(category),
            result,
            duration_ms,
        });
        self.recalculate_score();
    }

    fn recalculate_score(&mut self) {
        let total = self.passed + self.failed;
        if total > 0 {
            self.score = self.passed as f32 / total as f32;
        }
    }

    pub fn should_promote(&self) -> bool {
        self.score >= 0.9 && self.passed >= 10 && self.failed == 0
    }

    pub fn promote(&mut self) -> bool {
        if self.should_promote() {
            self.promoted = true;
            true
        } else {
            false
        }
    }

    pub fn failures(&self) -> Vec<&SandboxTest> {
        self.tests.iter().filter(|t| matches!(t.result, TestResult::Fail(_))).collect()
    }

    pub fn categories_tested(&self) -> Vec<&str> {
        let mut cats: Vec<&str> = self.tests.iter().map(|t| t.category.as_str()).collect();
        cats.dedup();
        cats
    }
}
