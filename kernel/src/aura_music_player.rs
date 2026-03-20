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

pub struct AuraMusicPlayer {
    playlist: Vec<String>,
    current_track_index: usize,
    is_playing: bool,
}

impl AuraMusicPlayer {
    pub fn new() -> Self {
        AuraMusicPlayer {
            playlist: Vec::new(),
            current_track_index: 0,
            is_playing: false,
        }
    }

    pub fn add_track(&mut self, track_name: String) {
        self.playlist.push(track_name);
    }

    pub fn remove_track(&mut self, index: usize) -> Option<String> {
        if index < self.playlist.len() {
            Some(self.playlist.remove(index))
        } else {
            None
        }
    }

    pub fn play(&mut self) {
        if !self.playlist.is_empty() && self.current_track_index < self.playlist.len() {
            self.is_playing = true;
            // Simulate playing the track
            println!("Playing: {}", self.playlist[self.current_track_index]);
        }
    }

    pub fn pause(&mut self) {
        self.is_playing = false;
        // Simulate pausing the track
        println!("Paused");
    }

    pub fn next_track(&mut self) {
        if !self.playlist.is_empty() {
            self.current_track_index = (self.current_track_index + 1) % self.playlist.len();
            if self.is_playing {
                self.play();
            }
        }
    }
}
