extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct AccessDwellClick {
    clicks: Vec<String>,
    dwell_times: Vec<u64>,
}

impl AccessDwellClick {
    pub fn new() -> Self {
        AccessDwellClick {
            clicks: Vec::new(),
            dwell_times: Vec::new(),
        }
    }

    pub fn record_click(&mut self, click_data: String) {
        self.clicks.push(click_data);
    }

    pub fn record_dwell_time(&mut self, time: u64) {
        self.dwell_times.push(time);
    }

    pub fn get_total_clicks(&self) -> usize {
        self.clicks.len()
    }

    pub fn get_average_dwell_time(&self) -> Option<u64> {
        if self.dwell_times.is_empty() {
            None
        } else {
            let total: u64 = self.dwell_times.iter().sum();
            Some(total / self.dwell_times.len() as u64)
        }
    }

    pub fn get_click_data(&self, index: usize) -> Option<&String> {
        self.clicks.get(index)
    }
}
