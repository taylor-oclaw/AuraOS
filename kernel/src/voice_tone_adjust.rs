extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct VoiceToneAdjust {
    tone_levels: Vec<i32>,
    current_tone: i32,
}

impl VoiceToneAdjust {
    pub fn new(initial_tone: i32) -> Self {
        let mut levels = Vec::new();
        for i in -10..=10 {
            levels.push(i);
        }
        VoiceToneAdjust {
            tone_levels: levels,
            current_tone: initial_tone,
        }
    }

    pub fn get_current_tone(&self) -> i32 {
        self.current_tone
    }

    pub fn set_tone(&mut self, new_tone: i32) {
        if self.tone_levels.contains(&new_tone) {
            self.current_tone = new_tone;
        }
    }

    pub fn increase_tone(&mut self) {
        let index = self.tone_levels.iter().position(|&x| x == self.current_tone).unwrap_or(0);
        if index < self.tone_levels.len() - 1 {
            self.current_tone = self.tone_levels[index + 1];
        }
    }

    pub fn decrease_tone(&mut self) {
        let index = self.tone_levels.iter().position(|&x| x == self.current_tone).unwrap_or(0);
        if index > 0 {
            self.current_tone = self.tone_levels[index - 1];
        }
    }

    pub fn reset_tone(&mut self) {
        self.current_tone = 0;
    }
}
