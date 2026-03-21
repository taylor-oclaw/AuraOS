extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct RelPromotionDetect {
    data: Vec<u8>,
    threshold: u8,
    results: Vec<bool>,
}

impl RelPromotionDetect {
    pub fn new(data: Vec<u8>, threshold: u8) -> Self {
        RelPromotionDetect {
            data,
            threshold,
            results: Vec::new(),
        }
    }

    pub fn analyze(&mut self) {
        self.results.clear();
        for &value in &self.data {
            self.results.push(value > self.threshold);
        }
    }

    pub fn get_results(&self) -> &[bool] {
        &self.results
    }

    pub fn set_threshold(&mut self, threshold: u8) {
        self.threshold = threshold;
    }

    pub fn count_above_threshold(&self) -> usize {
        self.results.iter().filter(|&&x| x).count()
    }
}
