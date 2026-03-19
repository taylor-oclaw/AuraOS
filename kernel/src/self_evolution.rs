extern crate alloc;
use alloc::string::ToString;
use alloc::string::String;
use alloc::vec::Vec;

pub struct BugReport {
    pub id: u64,
    pub description: String,
    pub stack_trace: Option<String>,
    pub module: String,
    pub severity: u8,
    pub detected_at: u64
}

pub struct ProposedFix {
    pub bug_id: u64,
    pub branch_name: String,
    pub diff: String,
    pub tested_locally: bool,
    pub test_passed: bool,
    pub pr_url: Option<String>,
    pub status: FixStatus
}

pub enum FixStatus {
    Analyzing,
    Fixing,
    Testing,
    PrCreated,
    Merged,
    Rejected
}

pub struct SelfEvolution {
    pub bugs: Vec<BugReport>,
    pub fixes: Vec<ProposedFix>,
    pub next_bug_id: u64,
    pub daily_pr_count: u32,
    pub max_daily_prs: u32,
    pub device_id: String
}

impl SelfEvolution {
    pub fn new(device_id: &str) -> Self {
        Self {
            bugs: Vec::new(),
            fixes: Vec::new(),
            next_bug_id: 1,
            daily_pr_count: 0,
            max_daily_prs: 1,
            device_id: String::from(device_id)
        }
    }

    pub fn report_bug(&mut self, desc: &str, module: &str, severity: u8) -> u64 {
        let id = self.next_bug_id;
        self.next_bug_id += 1;
        self.bugs.push(BugReport {
            id,
            description: String::from(desc),
            stack_trace: None,
            module: String::from(module),
            severity,
            detected_at: 0
        });
        id
    }

    pub fn propose_fix(&mut self, bug_id: u64, diff: &str) -> bool {
        if self.daily_pr_count >= self.max_daily_prs {
            return false;
        }
        let branch = String::from("fix/") + &self.device_id + "-" + &bug_id.to_string();
        self.fixes.push(ProposedFix {
            bug_id,
            branch_name: branch,
            diff: String::from(diff),
            tested_locally: false,
            test_passed: false,
            pr_url: None,
            status: FixStatus::Fixing
        });
        true
    }

    pub fn run_local_tests(&mut self, bug_id: u64, passed: bool) {
        if let Some(fix) = self.fixes.iter_mut().find(|f| f.bug_id == bug_id) {
            fix.tested_locally = true;
            fix.test_passed = passed;
            fix.status = if passed { FixStatus::Testing } else { FixStatus::Rejected };
        }
    }

    pub fn create_pr(&mut self, bug_id: u64, pr_url: &str) {
        if let Some(fix) = self.fixes.iter_mut().find(|f| f.bug_id == bug_id && f.test_passed) {
            fix.pr_url = Some(String::from(pr_url));
            fix.status = FixStatus::PrCreated;
            self.daily_pr_count += 1;
        }
    }

    pub fn can_create_pr(&self) -> bool {
        self.daily_pr_count < self.max_daily_prs
    }

    pub fn pending_fixes(&self) -> usize {
        self.fixes.iter().filter(|f| matches!(f.status, FixStatus::Testing)).count()
    }
}
