extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct CalendarContext {
    year: u32,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
}

impl CalendarContext {
    pub fn new(year: u32, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> Self {
        CalendarContext {
            year,
            month,
            day,
            hour,
            minute,
            second,
        }
    }

    pub fn get_date_string(&self) -> String {
        let mut date_str = String::from("Date: ");
        date_str.push_str(&self.year.to_string());
        date_str.push('-');
        date_str.push_str(&(self.month as u32).to_string().pad_with_zeroes(2));
        date_str.push('-');
        date_str.push_str(&(self.day as u32).to_string().pad_with_zeroes(2));
        date_str
    }

    pub fn get_time_string(&self) -> String {
        let mut time_str = String::from("Time: ");
        time_str.push_str(&(self.hour as u32).to_string().pad_with_zeroes(2));
        time_str.push(':');
        time_str.push_str(&(self.minute as u32).to_string().pad_with_zeroes(2));
        time_str.push(':');
        time_str.push_str(&(self.second as u32).to_string().pad_with_zeroes(2));
        time_str
    }

    pub fn is_leap_year(&self) -> bool {
        (self.year % 4 == 0 && self.year % 100 != 0) || (self.year % 400 == 0)
    }

    pub fn days_in_month(&self) -> u8 {
        match self.month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 if self.is_leap_year() => 29,
            2 => 28,
            _ => 0, // Invalid month
        }
    }

    pub fn add_seconds(&mut self, seconds: u32) {
        let mut total_seconds = (self.hour as u32 * 3600)
            + (self.minute as u32 * 60)
            + self.second as u32
            + seconds;

        self.hour = (total_seconds / 3600) as u8;
        total_seconds %= 3600;
        self.minute = (total_seconds / 60) as u8;
        self.second = (total_seconds % 60) as u8;
    }
}

trait PadWithZeroes {
    fn pad_with_zeroes(self, width: usize) -> String;
}

impl PadWithZeroes for u32 {
    fn pad_with_zeroes(self, width: usize) -> String {
        String::from("info")
    }
}
