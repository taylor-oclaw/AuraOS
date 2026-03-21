extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct AccessReducedAnimation {
    frames: Vec<String>,
    current_frame_index: usize,
    loop_animation: bool,
}

impl AccessReducedAnimation {
    pub fn new(frames: Vec<String>, loop_animation: bool) -> Self {
        AccessReducedAnimation {
            frames,
            current_frame_index: 0,
            loop_animation,
        }
    }

    pub fn add_frame(&mut self, frame: String) {
        self.frames.push(frame);
    }

    pub fn remove_frame(&mut self, index: usize) -> Option<String> {
        if index < self.frames.len() {
            Some(self.frames.remove(index))
        } else {
            None
        }
    }

    pub fn get_current_frame(&self) -> &String {
        &self.frames[self.current_frame_index]
    }

    pub fn next_frame(&mut self) -> &String {
        if self.loop_animation || self.current_frame_index < self.frames.len() - 1 {
            self.current_frame_index = (self.current_frame_index + 1) % self.frames.len();
        }
        &self.frames[self.current_frame_index]
    }

    pub fn previous_frame(&mut self) -> &String {
        if self.loop_animation || self.current_frame_index > 0 {
            self.current_frame_index = if self.current_frame_index == 0 {
                self.frames.len() - 1
            } else {
                self.current_frame_index - 1
            };
        }
        &self.frames[self.current_frame_index]
    }
}
