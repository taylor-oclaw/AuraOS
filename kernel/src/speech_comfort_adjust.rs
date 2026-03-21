extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct SpeechComfortAdjust {
    volume: u8,
    bass: u8,
    treble: u8,
    balance: i8,
    equalizer: Vec<u8>,
}

impl SpeechComfortAdjust {
    pub fn new() -> Self {
        SpeechComfortAdjust {
            volume: 50,
            bass: 50,
            treble: 50,
            balance: 0,
            equalizer: vec![50; 10],
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

    pub fn adjust_bass(&mut self, adjustment: i8) {
        let new_bass = (self.bass as i16 + adjustment).clamp(0, 100) as u8;
        self.bass = new_bass;
    }

    pub fn get_bass(&self) -> u8 {
        self.bass
    }

    pub fn adjust_treble(&mut self, adjustment: i8) {
        let new_treble = (self.treble as i16 + adjustment).clamp(0, 100) as u8;
        self.treble = new_treble;
    }

    pub fn get_treble(&self) -> u8 {
        self.treble
    }

    pub fn set_balance(&mut self, balance: i8) {
        self.balance = balance.clamp(-100, 100);
    }

    pub fn get_balance(&self) -> i8 {
        self.balance
    }

    pub fn adjust_equalizer_band(&mut self, band: usize, adjustment: i8) {
        if let Some(eq) = self.equalizer.get_mut(band) {
            *eq = (*eq as i16 + adjustment).clamp(0, 100) as u8;
        }
    }

    pub fn get_equalizer_band(&self, band: usize) -> Option<u8> {
        self.equalizer.get(band).cloned()
    }

    pub fn reset_settings(&mut self) {
        *self = SpeechComfortAdjust::new();
    }
}
