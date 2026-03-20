extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_ffi_init() {
    // Initialize the module
}

pub extern "C" fn rust_ffi_exit() {
    // Clean up the module
}

struct AuraCamera {
    name: String,
    resolution: (u32, u32),
    frame_rate: u32,
    is_active: bool,
    captured_frames: Vec<u8>,
}

impl AuraCamera {
    pub fn new(name: &str, resolution: (u32, u32), frame_rate: u32) -> Self {
        AuraCamera {
            name: String::from(name),
            resolution,
            frame_rate,
            is_active: false,
            captured_frames: Vec::new(),
        }
    }

    pub fn activate(&mut self) {
        self.is_active = true;
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    pub fn capture_frame(&mut self, frame_data: &[u8]) {
        if self.is_active {
            self.captured_frames.extend_from_slice(frame_data);
        }
    }

    pub fn get_last_frame(&self) -> Option<&[u8]> {
        if !self.captured_frames.is_empty() {
            Some(&self.captured_frames[self.captured_frames.len() - frame_size(self.resolution)..])
        } else {
            None
        }
    }

    pub fn get_frame_rate(&self) -> u32 {
        self.frame_rate
    }
}

fn frame_size(resolution: (u32, u32)) -> usize {
    (resolution.0 * resolution.1 * 3) as usize // Assuming RGB format
}
