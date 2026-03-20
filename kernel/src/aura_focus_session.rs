extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct AuraFocusSession {
    session_id: u32,
    user_id: String,
    active_windows: Vec<String>,
    last_active_time: u64,
    is_focused: bool,
}

impl AuraFocusSession {
    pub fn new(session_id: u32, user_id: &str) -> Self {
        AuraFocusSession {
            session_id,
            user_id: String::from(user_id),
            active_windows: Vec::new(),
            last_active_time: 0,
            is_focused: false,
        }
    }

    pub fn add_window(&mut self, window_title: &str) {
        if !self.active_windows.contains(&String::from(window_title)) {
            self.active_windows.push(String::from(window_title));
        }
    }

    pub fn remove_window(&mut self, window_title: &str) {
        self.active_windows.retain(|w| w != window_title);
    }

    pub fn get_active_windows(&self) -> Vec<String> {
        self.active_windows.clone()
    }

    pub fn set_focus(&mut self, is_focused: bool) {
        self.is_focused = is_focused;
        if is_focused {
            self.last_active_time = crate::kernel::get_current_time();
        }
    }

    pub fn is_session_focused(&self) -> bool {
        self.is_focused
    }
}
