extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct ParentalScreenTimeSchedule {
    user_id: u32,
    schedule: Vec<(u8, u8, u8, u8)>, // (day_of_week, start_hour, start_minute, duration_minutes)
}

impl ParentalScreenTimeSchedule {
    pub fn new(user_id: u32) -> Self {
        ParentalScreenTimeSchedule {
            user_id,
            schedule: Vec::new(),
        }
    }

    pub fn add_schedule(&mut self, day_of_week: u8, start_hour: u8, start_minute: u8, duration_minutes: u8) {
        if day_of_week < 7 && start_hour < 24 && start_minute < 60 && duration_minutes > 0 {
            self.schedule.push((day_of_week, start_hour, start_minute, duration_minutes));
        }
    }

    pub fn remove_schedule(&mut self, index: usize) -> bool {
        if index < self.schedule.len() {
            self.schedule.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_schedule(&self) -> &Vec<(u8, u8, u8, u8)> {
        &self.schedule
    }

    pub fn is_allowed_to_use_screen(&self, current_day: u8, current_hour: u8, current_minute: u8) -> bool {
        for &(day_of_week, start_hour, start_minute, duration_minutes) in &self.schedule {
            if day_of_week == current_day && start_hour <= current_hour && (start_hour < current_hour || start_minute <= current_minute) {
                let end_time = start_minute + duration_minutes;
                let end_hour = start_hour + end_time / 60;
                let end_minute = end_time % 60;

                if current_hour < end_hour || (current_hour == end_hour && current_minute < end_minute) {
                    return false;
                }
            }
        }
        true
    }

    pub fn get_user_id(&self) -> u32 {
        self.user_id
    }
}
