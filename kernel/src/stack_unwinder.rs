extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct StackUnwinder {
    // Placeholder for actual stack unwinding data structure
    frames: Vec<String>,
}

impl StackUnwinder {
    pub fn new() -> Self {
        StackUnwinder {
            frames: Vec::new(),
        }
    }

    pub fn add_frame(&mut self, frame: String) {
        self.frames.push(frame);
    }

    pub fn get_frames(&self) -> &Vec<String> {
        &self.frames
    }

    pub fn clear_frames(&mut self) {
        self.frames.clear();
    }

    pub fn has_frames(&self) -> bool {
        !self.frames.is_empty()
    }

    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }
}
