extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AnalyticsMonthlyReport {
    month: u8,
    year: u16,
    data: Vec<(String, u32)>,
}

impl AnalyticsMonthlyReport {
    pub fn new(month: u8, year: u16) -> Self {
        AnalyticsMonthlyReport {
            month,
            year,
            data: Vec::new(),
        }
    }

    pub fn add_data(&mut self, key: String, value: u32) {
        self.data.push((key, value));
    }

    pub fn get_total(&self) -> u32 {
        self.data.iter().map(|&(_, v)| v).sum()
    }

    pub fn get_average(&self) -> Option<f32> {
        if self.data.is_empty() {
            None
        } else {
            Some(self.get_total() as f32 / self.data.len() as f32)
        }
    }

    pub fn find_max(&self) -> Option<(String, u32)> {
        self.data.iter().cloned().max_by_key(|&(_, v)| v)
    }

    pub fn find_min(&self) -> Option<(String, u32)> {
        self.data.iter().cloned().min_by_key(|&(_, v)| v)
    }
}
