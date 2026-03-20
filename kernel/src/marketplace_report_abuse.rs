extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn marketplace_report_abuse_init() {
    // Initialization logic for the module
}

pub extern "C" fn marketplace_report_abuse_exit() {
    // Cleanup logic for the module
}

pub struct MarketplaceReportAbuse {
    reports: Vec<String>,
}

impl MarketplaceReportAbuse {
    pub fn new() -> Self {
        MarketplaceReportAbuse {
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
}
