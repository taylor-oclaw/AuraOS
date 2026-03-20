extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct SkillReportAbuse {
    reports: Vec<String>,
}

impl SkillReportAbuse {
    pub fn new() -> Self {
        SkillReportAbuse {
            reports: Vec::new(),
        }
    }

    pub fn add_report(&mut self, report: String) {
        self.reports.push(report);
    }

    pub fn get_reports_count(&self) -> usize {
        self.reports.len()
    }

    pub fn get_all_reports(&self) -> &[String] {
        &self.reports
    }

    pub fn clear_reports(&mut self) {
        self.reports.clear();
    }

    pub fn find_report(&self, keyword: &str) -> Vec<String> {
        let mut matching_reports = Vec::new();
        for report in &self.reports {
            if report.contains(keyword) {
                matching_reports.push(report.clone());
            }
        }
        matching_reports
    }
}
