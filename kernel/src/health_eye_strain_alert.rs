extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct HealthEyeStrainAlert {
    user_id: u32,
    session_start_time: u64,
    eye_strain_level: u8,
    alerts_sent: Vec<String>,
}

impl HealthEyeStrainAlert {
    pub fn new(user_id: u32) -> Self {
        HealthEyeStrainAlert {
            user_id,
            session_start_time: 0,
            eye_strain_level: 0,
            alerts_sent: Vec::new(),
        }
    }

    pub fn start_session(&mut self, current_time: u64) {
        self.session_start_time = current_time;
    }

    pub fn update_eye_strain_level(&mut self, new_level: u8) {
        if new_level > self.eye_strain_level {
            self.eye_strain_level = new_level;
        }
    }

    pub fn get_session_duration(&self, current_time: u64) -> u64 {
        if self.session_start_time == 0 {
            0
        } else {
            current_time - self.session_start_time
        }
    }

    pub fn send_alert(&mut self, alert_message: &str) {
        let message = String::from(alert_message);
        self.alerts_sent.push(message);
    }

    pub fn get_alert_history(&self) -> &[String] {
        &self.alerts_sent
    }
}
