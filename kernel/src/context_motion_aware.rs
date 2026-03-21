extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ContextMotionAware {
    motion_data: Vec<u8>,
    threshold: u8,
    state: String,
}

impl ContextMotionAware {
    pub fn new(threshold: u8) -> Self {
        ContextMotionAware {
            motion_data: Vec::new(),
            threshold,
            state: String::from("idle"),
        }
    }

    pub fn add_motion_data(&mut self, data: u8) {
        self.motion_data.push(data);
    }

    pub fn analyze_motion(&mut self) -> bool {
        let average = self.calculate_average();
        if average > self.threshold {
            self.state = String::from("active");
            true
        } else {
            self.state = String::from("idle");
            false
        }
    }

    fn calculate_average(&self) -> u8 {
        if self.motion_data.is_empty() {
            return 0;
        }
        let sum: u32 = self.motion_data.iter().map(|&x| x as u32).sum();
        (sum / self.motion_data.len() as u32) as u8
    }

    pub fn get_state(&self) -> &str {
        &self.state
    }

    pub fn clear_motion_data(&mut self) {
        self.motion_data.clear();
    }
}
