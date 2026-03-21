extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn game_mode_notify_silence_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn game_mode_notify_silence_exit() {
    // Cleanup logic for the module
}

pub struct GameModeNotifier {
    notifications: Vec<String>,
    is_silent: bool,
}

impl GameModeNotifier {
    pub fn new() -> Self {
        GameModeNotifier {
            notifications: Vec::new(),
            is_silent: false,
        }
    }

    pub fn add_notification(&mut self, message: &str) {
        if !self.is_silent {
            self.notifications.push(String::from(message));
        }
    }

    pub fn clear_notifications(&mut self) {
        self.notifications.clear();
    }

    pub fn set_silence_mode(&mut self, silent: bool) {
        self.is_silent = silent;
    }

    pub fn get_notifications(&self) -> &Vec<String> {
        &self.notifications
    }
}
