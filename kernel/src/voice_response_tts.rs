extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct VoiceResponseTTS {
    text: String,
    language: String,
    voice: String,
    speed: u8,
    volume: u8,
}

impl VoiceResponseTTS {
    pub fn new(text: &str, language: &str, voice: &str, speed: u8, volume: u8) -> Self {
        VoiceResponseTTS {
            text: String::from(text),
            language: String::from(language),
            voice: String::from(voice),
            speed,
            volume,
        }
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = String::from(text);
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn set_language(&mut self, language: &str) {
        self.language = String::from(language);
    }

    pub fn get_language(&self) -> &str {
        &self.language
    }

    pub fn set_voice(&mut self, voice: &str) {
        self.voice = String::from(voice);
    }

    pub fn get_voice(&self) -> &str {
        &self.voice
    }

    pub fn set_speed(&mut self, speed: u8) {
        self.speed = speed;
    }

    pub fn get_speed(&self) -> u8 {
        self.speed
    }

    pub fn set_volume(&mut self, volume: u8) {
        self.volume = volume;
    }

    pub fn get_volume(&self) -> u8 {
        self.volume
    }
}
