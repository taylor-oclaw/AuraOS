extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct DatePicker {
    year: u16,
    month: u8,
    day: u8,
}

impl DatePicker {
    pub fn new(year: u16, month: u8, day: u8) -> Self {
        DatePicker { year, month, day }
    }

    pub fn get_year(&self) -> u16 {
        self.year
    }

    pub fn set_year(&mut self, year: u16) {
        self.year = year;
    }

    pub fn get_month(&self) -> u8 {
        self.month
    }

    pub fn set_month(&mut self, month: u8) {
        if month > 0 && month <= 12 {
            self.month = month;
        }
    }

    pub fn get_day(&self) -> u8 {
        self.day
    }

    pub fn set_day(&mut self, day: u8) {
        match self.month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => {
                if day > 0 && day <= 31 {
                    self.day = day;
                }
            },
            4 | 6 | 9 | 11 => {
                if day > 0 && day <= 30 {
                    self.day = day;
                }
            },
            2 => {
                if (self.year % 4 == 0 && self.year % 100 != 0) || (self.year % 400 == 0) {
                    if day > 0 && day <= 29 {
                        self.day = day;
                    }
                } else if day > 0 && day <= 28 {
                    self.day = day;
                }
            },
            _ => {}
        }
    }

    pub fn is_leap_year(&self) -> bool {
        (self.year % 4 == 0 && self.year % 100 != 0) || (self.year % 400 == 0)
    }

    pub fn days_in_month(&self) -> u8 {
        match self.month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => if self.is_leap_year() { 29 } else { 28 },
            _ => 0,
        }
    }

    pub fn to_string(&self) -> String {
        let mut date_str = String::new();
        date_str.push_str(&format!("{:04}-{:02}-{:02}", self.year, self.month, self.day));
        date_str
    }
}
