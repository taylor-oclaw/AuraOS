extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub enum SoundEvent {
    Notification,
    Error,
    Warning,
    Success,
    Message,
    Call,
    Alarm,
    Startup,
    Shutdown,
    Lock,
    Unlock,
    Screenshot,
    KeyClick,
}

pub struct SoundProfile {
    pub name: String,
    pub enabled: bool,
    pub volume: u8,
    pub sounds: Vec<(SoundEvent, String)>,
}

pub struct SoundManager {
    pub profiles: Vec<SoundProfile>,
    pub active_profile: usize,
    pub global_mute: bool,
    pub do_not_disturb: bool,
}

impl SoundManager {
    pub fn new() -> Self {
        let default = SoundProfile {
            name: String::from("Default"),
            enabled: true,
            volume: 75,
            sounds: Vec::new(),
        };
        let silent = SoundProfile {
            name: String::from("Silent"),
            enabled: false,
            volume: 0,
            sounds: Vec::new(),
        };
        Self {
            profiles: vec![default, silent],
            active_profile: 0,
            global_mute: false,
            do_not_disturb: false,
        }
    }

    pub fn play(&self, _event: SoundEvent) -> bool {
        if self.global_mute || self.do_not_disturb {
            return false;
        }
        self.profiles[self.active_profile].enabled
    }

    pub fn set_volume(&mut self, vol: u8) {
        self.profiles[self.active_profile].volume = if vol > 100 { 100 } else { vol };
    }

    pub fn mute(&mut self) {
        self.global_mute = true;
    }

    pub fn unmute(&mut self) {
        self.global_mute = false;
    }

    pub fn set_dnd(&mut self, on: bool) {
        self.do_not_disturb = on;
    }

    pub fn switch_profile(&mut self, name: &str) {
        if let Some(idx) = self.profiles.iter().position(|p| p.name == name) {
            self.active_profile = idx;
        }
    }
}
