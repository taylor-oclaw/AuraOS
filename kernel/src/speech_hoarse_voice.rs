extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct SpeechHoarseVoice {
    voice_data: Vec<u8>,
    volume: u8,
    pitch: u8,
    speed: u8,
    effects: Vec<String>,
}

impl SpeechHoarseVoice {
    pub fn new() -> Self {
        SpeechHoarseVoice {
            voice_data: Vec::new(),
            volume: 50,
            pitch: 128,
            speed: 100,
            effects: Vec::new(),
        }
    }

    pub fn set_voice_data(&mut self, data: Vec<u8>) {
        self.voice_data = data;
    }

    pub fn get_voice_data(&self) -> &Vec<u8> {
        &self.voice_data
    }

    pub fn set_volume(&mut self, volume: u8) {
        if volume <= 100 {
            self.volume = volume;
        }
    }

    pub fn get_volume(&self) -> u8 {
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

    pub fn get_effects(&self) -> &Vec<String> {
        &self.effects
    }
}
