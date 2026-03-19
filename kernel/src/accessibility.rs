extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AccessibilitySettings {
    pub screen_reader: bool,
    pub high_contrast: bool,
    pub font_scale: f32,
    pub cursor_size: u8,
    pub reduce_motion: bool,
    pub voice_control: bool,
    pub captions: bool,
    pub color_blind_mode: Option<ColorBlindMode>
}

pub enum ColorBlindMode {
    Protanopia,
    Deuteranopia,
    Tritanopia
}

pub struct AccessibilityManager {
    pub settings: AccessibilitySettings,
    pub screen_reader_text: Vec<String>
}

impl AccessibilityManager {
    pub fn new() -> Self {
        Self {
            settings: AccessibilitySettings {
                screen_reader: false,
                high_contrast: false,
                font_scale: 1.0,
                cursor_size: 1,
                reduce_motion: false,
                voice_control: false,
                captions: false,
                color_blind_mode: None
            },
            screen_reader_text: Vec::new()
        }
    }

    pub fn enable_screen_reader(&mut self) {
        self.settings.screen_reader = true;
    }

    pub fn set_font_scale(&mut self, scale: f32) {
        self.settings.font_scale = if scale < 0.5 { 0.5 } else if scale > 4.0 { 4.0 } else { scale };
    }

    pub fn enable_high_contrast(&mut self) {
        self.settings.high_contrast = true;
    }

    pub fn announce(&mut self, text: &str) {
        if self.settings.screen_reader {
            self.screen_reader_text.push(String::from(text));
        }
    }

    pub fn next_announcement(&mut self) -> Option<String> {
        if self.screen_reader_text.is_empty() {
            None
        } else {
            Some(self.screen_reader_text.remove(0))
        }
    }

    pub fn needs_large_text(&self) -> bool {
        self.settings.font_scale > 1.5
    }
}
