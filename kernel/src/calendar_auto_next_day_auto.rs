extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct CalendarAutoNextDayAuto {
    year: u32,
    month: u8,
    day: u8,
}

impl CalendarAutoNextDayAuto {
    pub fn new(year: u32, month: u8, day: u8) -> Self {
        CalendarAutoNextDayAuto { year, month, day }
    }

    pub fn get_year(&self) -> u32 {
        self.year
    }

    pub fn get_month(&self) -> u8 {
        self.month
    }

    pub fn get_day(&self) -> u8 {
        self.day
    }

    pub fn is_leap_year(&self) -> bool {
        (self.year % 4 == 0 && self.year % 100 != 0) || self.year % 400 == 0
    }

    pub fn days_in_month(&self, month: u8) -> u32 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => if self.is_leap_year() { 29 } else { 28 },
            _ => panic!("Invalid month"),
        }
    }

    pub fn next_day(&self) -> (u32, u8, u8) {
        let mut year = self.year;
        let mut month = self.month;
        let mut day = self.day;

        if day == self.days_in_month(month) {
            if month == 12 {
                year += 1;
                month = 1;
                day = 1;
            } else {
                month += 1;
                day = 1;
            }
        } else {
            day += 1;
        }

        (year, month, day)
    }
}