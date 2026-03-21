extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct LangTtsEmotionVoice {
    language: String,
    text: String,
    emotion: String,
    voice: String,
    volume: u8,
}

impl LangTtsEmotionVoice {
    pub fn new(language: &str, text: &str, emotion: &str, voice: &str, volume: u8) -> Self {
        LangTtsEmotionVoice {
            language: String::from(language),
            text: String::from(text),
            emotion: String::from(emotion),
            voice: String::from(voice),
            volume,
        }
    }

    pub fn set_language(&mut self, language: &str) {
        self.language = String::from(language);
    }

    pub fn get_language(&self) -> &str {
        &self.language
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = String::from(text);
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn set_emotion(&mut self, emotion: &str) {
        self.emotion = String::from(emotion);
    }

    pub fn get_emotion(&self) -> &str {
        &self.emotion
    }

    pub fn set_voice(&mut self, voice: &str) {
        self.voice = String::from(voice);
    }

    pub fn get_voice(&self) -> &str {
        &self.voice
    }

    pub fn set_volume(&mut self, volume: u8) {
        self.volume = volume;
    }

    pub fn get_volume(&self) -> u8 {
        self.volume
    }
}
