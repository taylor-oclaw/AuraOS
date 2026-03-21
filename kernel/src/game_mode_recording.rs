extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
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
            println!("Recording started.");
        } else {
            println!("Recording already active.");
        }
    }

    pub fn stop_recording(&mut self) {
        if self.is_active {
            self.is_active = false;
            println!("Recording stopped.");
        } else {
            println!("No recording to stop.");
        }
    }

    pub fn add_event(&mut self, event: String) {
        if self.is_active {
            self.recording.push(event);
            println!("Event recorded: {}", event);
        } else {
            println!("Recording is not active. Cannot record event.");
        }
    }

    pub fn get_recording(&self) -> Vec<String> {
        self.recording.clone()
    }

    pub fn clear_recording(&mut self) {
        if self.is_active {
            self.recording.clear();
            println!("Recording cleared.");
        } else {
            println!("Cannot clear recording while it is not active.");
        }
    }
}
