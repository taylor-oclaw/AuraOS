extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    let mut alert = AccessFlashScreenAlert::new();
    alert.set_message("AI System Alert".to_string());
    alert.set_duration(10);
    alert.set_color((255, 0, 0)); // Red color
    alert.enable_sound(true);
    alert.show_alert();

    loop {}
}

pub struct AccessFlashScreenAlert {
    message: String,
    duration: u32,
    color: (u8, u8, u8),
    sound_enabled: bool,
}

impl AccessFlashScreenAlert {
    pub fn new() -> Self {
        AccessFlashScreenAlert {
            message: String::from("Default Alert"),
            duration: 5,
            color: (0, 255, 0), // Green color
            sound_enabled: false,
        }
    }

    pub fn set_message(&mut self, message: String) {
        self.message = message;
    }

    pub fn get_message(&self) -> &str {
        &self.message
    }

    pub fn set_duration(&mut self, duration: u32) {
        self.duration = duration;
    }

    pub fn get_duration(&self) -> u32 {
        self.duration
    }

    pub fn set_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    pub fn get_color(&self) -> (u8, u8, u8) {
        self.color
    }

    pub fn enable_sound(&mut self, enabled: bool) {
        self.sound_enabled = enabled;
    }

    pub fn is_sound_enabled(&self) -> bool {
        self.sound_enabled
    }

    pub fn show_alert(&self) {
        // Simulate showing an alert on the screen with given parameters
        if self.sound_enabled {
        } else {
        }
    }
}
