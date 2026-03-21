extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    let mut audio_system = SmartHomeAudioAnnounce::new();
    audio_system.set_volume(75);
    audio_system.play_announcement("Welcome home!");
    println!("Volume: {}", audio_system.get_volume());
    println!("Is playing: {}", audio_system.is_playing());
    audio_system.pause();
    println!("Is paused: {}", audio_system.is_paused());
    audio_system.stop();
}

pub struct SmartHomeAudioAnnounce {
    volume: u8,
    is_playing: bool,
    is_paused: bool,
    announcements: Vec<String>,
}

impl SmartHomeAudioAnnounce {
    pub fn new() -> Self {
        SmartHomeAudioAnnounce {
            volume: 50,
            is_playing: false,
            is_paused: false,
            announcements: Vec::new(),
        }
    }

    pub fn set_volume(&mut self, volume: u8) {
        if volume <= 100 {
            self.volume = volume;
        }
    }

    pub fn get_volume(&self) -> u8 {
        self.volume
    }

    pub fn play_announcement(&mut self, announcement: &str) {
        self.announcements.push(String::from(announcement));
        self.is_playing = true;
        self.is_paused = false;
    }

    pub fn is_playing(&self) -> bool {
        self.is_playing
    }

    pub fn pause(&mut self) {
        if self.is_playing {
            self.is_paused = true;
            self.is_playing = false;
        }
    }

    pub fn is_paused(&self) -> bool {
        self.is_paused
    }

    pub fn stop(&mut self) {
        self.is_playing = false;
        self.is_paused = false;
    }
}
