extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn meeting_app_detect_init() {
    // Initialization logic for the module
}

pub extern "C" fn meeting_app_detect_exit() {
    // Cleanup logic for the module
}

pub struct MeetingAppDetector {
    detected_apps: Vec<String>,
    active_meetings: usize,
}

impl MeetingAppDetector {
    pub fn new() -> Self {
        MeetingAppDetector {
            detected_apps: Vec::new(),
            active_meetings: 0,
        }
    }

    pub fn add_detected_app(&mut self, app_name: &str) {
        if !self.detected_apps.contains(&String::from(app_name)) {
            self.detected_apps.push(String::from(app_name));
        }
    }

    pub fn remove_detected_app(&mut self, app_name: &str) {
        if let Some(index) = self.detected_apps.iter().position(|x| x == app_name) {
            self.detected_apps.remove(index);
        }
    }

    pub fn is_app_detected(&self, app_name: &str) -> bool {
        self.detected_apps.contains(&String::from(app_name))
    }

    pub fn start_meeting(&mut self) {
        self.active_meetings += 1;
    }

    pub fn end_meeting(&mut self) {
        if self.active_meetings > 0 {
            self.active_meetings -= 1;
        }
    }

    pub fn get_active_meetings_count(&self) -> usize {
        self.active_meetings
    }

    pub fn list_detected_apps(&self) -> Vec<String> {
        self.detected_apps.clone()
    }
}
