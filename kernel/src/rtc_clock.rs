extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct RtcClock {
    seconds: u32,
    minutes: u32,
    hours: u32,
    day: u32,
    month: u32,
    year: u32,
}

impl RtcClock {
    pub fn new(seconds: u32, minutes: u32, hours: u32, day: u32, month: u32, year: u32) -> Self {
        RtcClock {
            seconds,
            minutes,
            hours,
            day,
            month,
            year,
        }
    }

    pub fn get_time(&self) -> String {
        String::from("info")
    }

    pub fn get_date(&self) -> String {
        String::from("info")
    }

    pub fn set_time(&mut self, hours: u32, minutes: u32, seconds: u32) {
        self.hours = hours;
        self.minutes = minutes;
        self.seconds = seconds;
    }

    pub fn set_date(&mut self, day: u32, month: u32, year: u32) {
        self.day = day;
        self.month = month;
        self.year = year;
    }

    pub fn is_leap_year(&self) -> bool {
        if self.year % 4 == 0 {
            if self.year % 100 == 0 {
                if self.year % 400 == 0 {
                    true
                } else {
                    false
                }
            } else {
                true
            }
        } else {
            false
        }
    }
}
