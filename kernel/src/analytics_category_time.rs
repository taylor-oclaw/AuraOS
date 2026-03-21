extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AnalyticsCategoryTime {
    category: String,
    times: Vec<u64>,
}

impl AnalyticsCategoryTime {
    pub fn new(category: &str) -> Self {
        AnalyticsCategoryTime {
            category: String::from(category),
            times: Vec::new(),
        }
    }

    pub fn add_time(&mut self, time: u64) {
        self.times.push(time);
    }

    pub fn get_category(&self) -> &String {
        &self.category
    }

    pub fn get_times(&self) -> &Vec<u64> {
        &self.times
    }

    pub fn average_time(&self) -> Option<u64> {
        if self.times.is_empty() {
            None
        } else {
            let total: u64 = self.times.iter().sum();
            Some(total / self.times.len() as u64)
        }
    }

    pub fn max_time(&self) -> Option<u64> {
        self.times.iter().max().copied()
    }

    pub fn min_time(&self) -> Option<u64> {
        self.times.iter().min().copied()
    }
}
