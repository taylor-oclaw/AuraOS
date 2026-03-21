extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn init_module() -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn cleanup_module() -> i32 {
    0
}

pub struct TouchpointTracker {
    touchpoints: Vec<String>,
}

impl TouchpointTracker {
    pub fn new() -> Self {
        TouchpointTracker {
            touchpoints: Vec::new(),
        }
    }

    pub fn add_touchpoint(&mut self, touchpoint: String) {
        self.touchpoints.push(touchpoint);
    }

    pub fn remove_touchpoint(&mut self, index: usize) -> Option<String> {
        if index < self.touchpoints.len() {
            Some(self.touchpoints.remove(index))
        } else {
            None
        }
    }

    pub fn get_touchpoint(&self, index: usize) -> Option<&String> {
        self.touchpoints.get(index)
    }

    pub fn count_touchpoints(&self) -> usize {
        self.touchpoints.len()
    }

    pub fn list_touchpoints(&self) -> Vec<String> {
        self.touchpoints.clone()
    }
}
