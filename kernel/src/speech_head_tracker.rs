extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn speech_head_tracker_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn speech_head_tracker_exit() {
    // Cleanup logic for the module
}

pub struct SpeechHeadTracker {
    head_positions: Vec<(i32, i32)>,
    current_position: (i32, i32),
    threshold: u32,
}

impl SpeechHeadTracker {
    pub fn new(threshold: u32) -> Self {
        SpeechHeadTracker {
            head_positions: Vec::new(),
            current_position: (0, 0),
            threshold,
        }
    }

    pub fn update_head_position(&mut self, x: i32, y: i32) {
        self.head_positions.push((x, y));
        self.current_position = (x, y);
    }

    pub fn get_current_position(&self) -> (i32, i32) {
        self.current_position
    }

    pub fn has_moved_significantly(&self) -> bool {
        if let Some(last_position) = self.head_positions.last() {
            let distance_squared = (last_position.0 - self.current_position.0).pow(2)
                + (last_position.1 - self.current_position.1).pow(2);
            distance_squared > (self.threshold as i32).pow(2)
        } else {
            false
        }
    }

    pub fn reset(&mut self) {
        self.head_positions.clear();
        self.current_position = (0, 0);
    }

    pub fn get_head_positions(&self) -> Vec<(i32, i32)> {
        self.head_positions.clone()
    }
}
