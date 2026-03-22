extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_ffi_init() {
    // Initialize the module
}

#[no_mangle]
pub extern "C" fn rust_ffi_exit() {
    // Cleanup the module
}

struct AuraVideoPlayer {
    video_files: Vec<String>,
    current_index: usize,
    is_playing: bool,
}

impl AuraVideoPlayer {
    pub fn new() -> Self {
        AuraVideoPlayer {
            video_files: Vec::new(),
            current_index: 0,
            is_playing: false,
        }
    }

    pub fn add_video(&mut self, file_name: String) {
        self.video_files.push(file_name);
    }

    pub fn play(&mut self) {
        if !self.video_files.is_empty() && self.current_index < self.video_files.len() {
            // Simulate playing the video
            self.is_playing = true;
            println!("Playing: {}", self.video_files[self.current_index]);
        }
    }

    pub fn pause(&mut self) {
        self.is_playing = false;
        println!("Paused");
    }

    pub fn next(&mut self) {
        if !self.video_files.is_empty() {
            self.current_index = (self.current_index + 1) % self.video_files.len();
            if self.is_playing {
                // Simulate playing the next video
                println!("Playing: {}", self.video_files[self.current_index]);
            }
        }
    }

    pub fn previous(&mut self) {
        if !self.video_files.is_empty() {
            if self.current_index == 0 {
                self.current_index = self.video_files.len() - 1;
            } else {
                self.current_index -= 1;
            }
            if self.is_playing {
                // Simulate playing the previous video
                println!("Playing: {}", self.video_files[self.current_index]);
            }
        }
    }

    pub fn stop(&mut self) {
        self.is_playing = false;
        self.current_index = 0;
        println!("Stopped");
    }
}