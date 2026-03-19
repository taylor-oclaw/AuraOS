extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub struct DateTime {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub utc_offset_mins: i16
}

pub struct Timezone {
    pub name: String,
    pub offset_mins: i16
}

pub struct DateTimeManager {
    pub current: DateTime,
    pub timezone: Timezone,
    pub use_24h: bool,
    pub auto_sync: bool,
    pub timezones: Vec<Timezone>
}

impl DateTimeManager {
    pub fn new() -> Self {
        Self {
            current: DateTime {
                year: 2026,
                month: 1,
                day: 1,
                hour: 0,
                minute: 0,
                second: 0,
                utc_offset_mins: 0
            },
            timezone: Timezone {
                name: String::from("UTC"),
                offset_mins: 0
            },
            use_24h: false,
            auto_sync: true,
            timezones: vec![
                Timezone { name: String::from("UTC"), offset_mins: 0 },
                Timezone { name: String::from("US/Eastern"), offset_mins: -300 },
                Timezone { name: String::from("US/Central"), offset_mins: -360 },
                Timezone { name: String::from("US/Pacific"), offset_mins: -480 },
                Timezone { name: String::from("Europe/London"), offset_mins: 0 },
                Timezone { name: String::from("Asia/Tokyo"), offset_mins: 540 }
            ]
        }
    }

    pub fn set_timezone(&mut self, name: &str) {
        if let Some(tz) = self.timezones.iter().find(|t| t.name == name) {
            self.timezone = Timezone { name: tz.name.clone(), offset_mins: tz.offset_mins };
            self.current.utc_offset_mins = tz.offset_mins;
        }
    }

    pub fn tick(&mut self) {
        self.current.second += 1;
        if self.current.second >= 60 {
            self.current.second = 0;
            self.current.minute += 1;
        }
        if self.current.minute >= 60 {
            self.current.minute = 0;
            self.current.hour += 1;
        }
        if self.current.hour >= 24 {
            self.current.hour = 0;
            self.current.day += 1;
        }
    }

    pub fn formatted_time(&self) -> String {
        if self.use_24h {
            String::new()
        } else {
            let hour = if self.current.hour == 0 { 12 } else if self.current.hour > 12 { self.current.hour - 12 } else { self.current.hour };
            let period = if self.current.hour >= 12 { "PM" } else { "AM" };
            String::new()
        }
    }
}
