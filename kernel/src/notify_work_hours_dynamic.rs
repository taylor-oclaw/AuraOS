extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct WorkHoursNotifier {
    start_time: u32,
    end_time: u32,
    notifications: Vec<String>,
}

impl WorkHoursNotifier {
    pub fn new(start_time: u32, end_time: u32) -> Self {
        WorkHoursNotifier {
            start_time,
            end_time,
            notifications: Vec::new(),
        }
    }

    pub fn add_notification(&mut self, notification: String) {
        self.notifications.push(notification);
    }

    pub fn remove_notification(&mut self, index: usize) -> Option<String> {
        if index < self.notifications.len() {
            Some(self.notifications.remove(index))
        } else {
            None
        }
    }

    pub fn get_notifications(&self) -> &[String] {
        &self.notifications
    }

    pub fn is_within_work_hours(&self, current_time: u32) -> bool {
        current_time >= self.start_time && current_time < self.end_time
    }

    pub fn notify_all(&self) -> Vec<String> {
        self.notifications.clone()
    }
}
