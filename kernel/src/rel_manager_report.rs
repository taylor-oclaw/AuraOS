extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct RelManagerReport {
    reports: Vec<String>,
}

impl RelManagerReport {
    pub fn new() -> Self {
        RelManagerReport {
            reports: Vec::new(),
        }
    }

    pub fn add_report(&mut self, report: String) {
        self.reports.push(report);
    }

    pub fn get_reports(&self) -> &Vec<String> {
        &self.reports
    }

    pub fn clear_reports(&mut self) {
        self.reports.clear();
    }

    pub fn has_reports(&self) -> bool {
        !self.reports.is_empty()
    }

    pub fn report_count(&self) -> usize {
        self.reports.len()
    }
}
