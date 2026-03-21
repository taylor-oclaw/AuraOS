extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct AnalyticsPeakHours {
    data: Vec<u32>,
}

impl AnalyticsPeakHours {
    pub fn new() -> Self {
        AnalyticsPeakHours { data: Vec::new() }
    }

    pub fn add_hour(&mut self, hour: u32) {
        if hour < 24 {
            self.data.push(hour);
        }
    }

    pub fn get_peak_hours(&self) -> Vec<u32> {
        let mut peak_hours = Vec::new();
        let mut max_count = 0;
        for hour in 0..24 {
            let count = self.data.iter().filter(|&&h| h == hour).count();
            if count > max_count {
                max_count = count;
                peak_hours.clear();
                peak_hours.push(hour);
            } else if count == max_count {
                peak_hours.push(hour);
            }
        }
        peak_hours
    }

    pub fn get_hour_count(&self, hour: u32) -> usize {
        self.data.iter().filter(|&&h| h == hour).count()
    }

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn total_entries(&self) -> usize {
        self.data.len()
    }
}
