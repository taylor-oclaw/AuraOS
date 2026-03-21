extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ProfileAccountPersonalEmail {
    email: String,
    verified: bool,
    notifications_enabled: bool,
    last_login_time: u64,
    login_count: u32,
}

impl ProfileAccountPersonalEmail {
    pub fn new(email: &str) -> Self {
        ProfileAccountPersonalEmail {
            email: String::from(email),
            verified: false,
            notifications_enabled: true,
            last_login_time: 0,
            login_count: 0,
        }
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }

    pub fn set_email(&mut self, email: &str) {
        self.email = String::from(email);
    }

    pub fn is_verified(&self) -> bool {
        self.verified
    }

    pub fn verify_email(&mut self) {
        self.verified = true;
    }

    pub fn toggle_notifications(&mut self) {
        self.notifications_enabled = !self.notifications_enabled;
    }

    pub fn record_login(&mut self, current_time: u64) {
        self.last_login_time = current_time;
        self.login_count += 1;
    }

    pub fn get_last_login_time(&self) -> u64 {
        self.last_login_time
    }

    pub fn get_login_count(&self) -> u32 {
        self.login_count
    }
}
