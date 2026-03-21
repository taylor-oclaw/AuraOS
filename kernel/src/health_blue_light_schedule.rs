extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use core::time::Duration;

pub struct HealthBlueLightSchedule {
    schedule: Vec<(u8, u8)>, // (hour, minute)
}

impl HealthBlueLightSchedule {
    pub fn new() -> Self {
        HealthBlueLightSchedule {
            schedule: Vec::new(),
        }
    }

    pub fn add_schedule(&mut self, hour: u8, minute: u8) {
        if hour < 24 && minute < 60 {
            self.schedule.push((hour, minute));
        }
    }

    pub fn remove_schedule(&mut self, hour: u8, minute: u8) -> bool {
        let pos = self.schedule.iter().position(|&(h, m)| h == hour && m == minute);
        if let Some(index) = pos {
            self.schedule.remove(index);
            true
        } else {
            false
        }
    }

    pub fn is_time_in_schedule(&self, current_hour: u8, current_minute: u8) -> bool {
        self.schedule.iter().any(|&(h, m)| h == current_hour && m == current_minute)
    }

    pub fn next_schedule_time(&self, current_hour: u8, current_minute: u8) -> Option<Duration> {
        let mut next_time = None;
        for &(hour, minute) in &self.schedule {
            if hour > current_hour || (hour == current_hour && minute > current_minute) {
                let hours_diff = hour as u64 - current_hour as u64;
                let minutes_diff = minute as u64 - current_minute as u64;
                let total_minutes = hours_diff * 60 + minutes_diff;
                next_time = Some(Duration::from_secs(total_minutes * 60));
                break;
            }
        }
        next_time
    }

    pub fn schedule_count(&self) -> usize {
        self.schedule.len()
    }
}
