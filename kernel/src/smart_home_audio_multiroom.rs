extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut audio_system = SmartHomeAudioMultiroom::new();
    audio_system.add_room("Living Room");
    audio_system.add_room("Bedroom");
    audio_system.set_volume(50);
    audio_system.play_song("Song1");
    audio_system.pause_song();
}

pub struct SmartHomeAudioMultiroom {
    rooms: Vec<String>,
    current_volume: u8,
    is_playing: bool,
    current_song: Option<String>,
}

impl SmartHomeAudioMultiroom {
    pub fn new() -> Self {
        SmartHomeAudioMultiroom {
            rooms: Vec::new(),
            current_volume: 50,
            is_playing: false,
            current_song: None,
        }
    }

    pub fn add_room(&mut self, room_name: &str) {
        self.rooms.push(String::from(room_name));
    }

    pub fn remove_room(&mut self, room_name: &str) -> bool {
        if let Some(index) = self.rooms.iter().position(|r| r == room_name) {
            self.rooms.remove(index);
            true
        } else {
            false
        }
    }

    pub fn set_volume(&mut self, volume: u8) {
        if volume <= 100 {
            self.current_volume = volume;
        }
    }

    pub fn play_song(&mut self, song_name: &str) {
        self.current_song = Some(String::from(song_name));
        self.is_playing = true;
    }

    pub fn pause_song(&mut self) {
        self.is_playing = false;
    }

    pub fn get_status(&self) -> String {
        let mut status = format!("Volume: {}", self.current_volume);
        if self.is_playing {
            status.push_str(", Playing: ");
            if let Some(song) = &self.current_song {
                status.push_str(song);
            }
        } else {
            status.push_str(", Paused");
        }
        status
    }
}
