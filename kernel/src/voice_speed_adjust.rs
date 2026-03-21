extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct VoiceSpeedAdjust {
    speed: u32,
    pitch: u32,
    volume: u32,
    effects: Vec<String>,
}

impl VoiceSpeedAdjust {
    pub fn new(speed: u32, pitch: u32, volume: u32) -> Self {
        VoiceSpeedAdjust {
            speed,
            pitch,
            volume,
            effects: Vec::new(),
        }
    }

    pub fn set_speed(&mut self, speed: u32) {
        self.speed = speed;
    }

    pub fn get_speed(&self) -> u32 {
        self.speed
    }

    pub fn set_pitch(&mut self, pitch: u32) {
        self.pitch = pitch;
    }

    pub fn get_pitch(&self) -> u32 {
        self.pitch
    }

    pub fn set_volume(&mut self, volume: u32) {
        self.volume = volume;
    }

    pub fn get_volume(&self) -> u32 {
        self.volume
    }

    pub fn add_effect(&mut self, effect: String) {
        self.effects.push(effect);
    }

    pub fn remove_effect(&mut self, index: usize) -> Option<String> {
        if index < self.effects.len() {
            Some(self.effects.remove(index))
        } else {
            None
        }
    }

    pub fn list_effects(&self) -> &[String] {
        &self.effects
    }
}
