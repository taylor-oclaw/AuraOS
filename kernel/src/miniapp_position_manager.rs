extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct PositionManager {
    positions: Vec<(i32, i32)>,
}

impl PositionManager {
    pub fn new() -> Self {
        PositionManager {
            positions: Vec::new(),
        }
    }

    pub fn add_position(&mut self, x: i32, y: i32) {
        self.positions.push((x, y));
    }

    pub fn remove_position(&mut self, index: usize) -> Option<(i32, i32)> {
        if index < self.positions.len() {
            Some(self.positions.remove(index))
        } else {
            None
        }
    }

    pub fn get_position(&self, index: usize) -> Option<&(i32, i32)> {
        self.positions.get(index)
    }

    pub fn count_positions(&self) -> usize {
        self.positions.len()
    }

    pub fn clear_positions(&mut self) {
        self.positions.clear();
    }
}
