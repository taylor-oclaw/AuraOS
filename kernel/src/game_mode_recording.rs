extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut game_mode = GameModeRecording::new();
    game_mode.start_recording();
    loop {}
}

pub struct GameModeRecording {
    recording: Vec<String>,
    is_active: bool,
}

impl GameModeRecording {
    pub fn new() -> Self {
        GameModeRecording {
            recording: Vec::new(),
            is_active: false,
        }
    }

    pub fn start_recording(&mut self) {
        if !self.is_active {
            self.is_active = true;
        } else {
        }
    }

    pub fn stop_recording(&mut self) {
        if self.is_active {
            self.is_active = false;
        } else {
        }
    }

    pub fn add_event(&mut self, event: String) {
        if self.is_active {
            self.recording.push(event);
        } else {
        }
    }

    pub fn get_recording(&self) -> Vec<String> {
        self.recording.clone()
    }

    pub fn clear_recording(&mut self) {
        if self.is_active {
            self.recording.clear();
        } else {
        }
    }
}
