extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct NotifyTimezoneTravelAdjust {
    user_id: u32,
    current_timezone: String,
    target_timezone: String,
    travel_date: String,
    notification_sent: bool,
}

impl NotifyTimezoneTravelAdjust {
    pub fn new(user_id: u32, current_timezone: &str, target_timezone: &str, travel_date: &str) -> Self {
        NotifyTimezoneTravelAdjust {
            user_id,
            current_timezone: String::from(current_timezone),
            target_timezone: String::from(target_timezone),
            travel_date: String::from(travel_date),
            notification_sent: false,
        }
    }

    pub fn get_user_id(&self) -> u32 {
        self.user_id
    }

    pub fn get_current_timezone(&self) -> &str {
        &self.current_timezone
    }

    pub fn get_target_timezone(&self) -> &str {
        &self.target_timezone
    }

    pub fn get_travel_date(&self) -> &str {
        &self.travel_date
    }

    pub fn is_notification_sent(&self) -> bool {
        self.notification_sent
    }

    pub fn send_notification(&mut self) {
        // Simulate sending a notification
        self.notification_sent = true;
        // In a real scenario, this would involve kernel-specific logic to send notifications
    }
}
