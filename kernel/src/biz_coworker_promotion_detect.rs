extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn init_module() -> i32 {
    0
}

pub extern "C" fn cleanup_module() -> i32 {
    0
}

pub struct CoworkerPromotionDetector {
    coworker_names: Vec<String>,
    promotion_threshold: usize,
}

impl CoworkerPromotionDetector {
    pub fn new(promotion_threshold: usize) -> Self {
        CoworkerPromotionDetector {
            coworker_names: Vec::new(),
            promotion_threshold,
        }
    }

    pub fn add_coworker(&mut self, name: String) {
        self.coworker_names.push(name);
    }

    pub fn remove_coworker(&mut self, name: &str) -> bool {
        if let Some(index) = self.coworker_names.iter().position(|n| n == name) {
            self.coworker_names.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_coworkers(&self) -> &[String] {
        &self.coworker_names
    }

    pub fn is_promotion_needed(&self) -> bool {
        self.coworker_names.len() >= self.promotion_threshold
    }

    pub fn reset(&mut self) {
        self.coworker_names.clear();
    }
}
