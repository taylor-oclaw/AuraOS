extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn miniapp_animation_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn miniapp_animation_exit() {
    // Cleanup logic for the module
}

pub struct Animation {
    frames: Vec<String>,
    current_frame: usize,
    looped: bool,
}

impl Animation {
    pub fn new(frames: Vec<String>, looped: bool) -> Self {
        Animation {
            frames,
            current_frame: 0,
            looped,
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
        &self.frames[self.current_frame]
    }

    pub fn next_frame(&mut self) -> &String {
        let current = &self.frames[self.current_frame];
        if self.current_frame + 1 < self.frames.len() {
            self.current_frame += 1;
        } else if self.looped {
            self.current_frame = 0;
        }
        current
    }

    pub fn previous_frame(&mut self) -> &String {
        let current = &self.frames[self.current_frame];
        if self.current_frame > 0 {
            self.current_frame -= 1;
        } else if self.looped && !self.frames.is_empty() {
            self.current_frame = self.frames.len() - 1;
        }
        current
    }

    pub fn set_looped(&mut self, looped: bool) {
        self.looped = looped;
    }

    pub fn is_looped(&self) -> bool {
        self.looped
    }
}
