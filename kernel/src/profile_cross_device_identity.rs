extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct ProfileCrossDeviceContextCarry {
    device_id: String,
    user_data: Vec<u8>,
    session_token: String,
    last_activity_time: u64,
    is_active: bool,
}

impl ProfileCrossDeviceContextCarry {
    pub fn new(device_id: &str, user_data: &[u8], session_token: &str) -> Self {
        ProfileCrossDeviceContextCarry {
            device_id: String::from(device_id),
            user_data: Vec::from(user_data),
            session_token: String::from(session_token),
            last_activity_time: 0,
            is_active: false,
        }
    }

    pub fn update_last_activity(&mut self, current_time: u64) {
        self.last_activity_time = current_time;
    }

    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }

    pub fn get_device_id(&self) -> &str {
        &self.device_id
    }

    pub fn get_user_data(&self) -> &[u8] {
        &self.user_data
    }

    pub fn get_session_token(&self) -> &str {
        &self.session_token
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }
}
