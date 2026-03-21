extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct ProfileTravelPerDiemTrack {
    name: String,
    days_traveled: u32,
    daily_allowance: u32,
    expenses: Vec<u32>,
}

impl ProfileTravelPerDiemTrack {
    pub fn new(name: &str, daily_allowance: u32) -> Self {
        ProfileTravelPerDiemTrack {
            name: String::from(name),
            days_traveled: 0,
            daily_allowance,
            expenses: Vec::new(),
        }
    }

    pub fn add_day(&mut self) {
        self.days_traveled += 1;
    }

    pub fn record_expense(&mut self, amount: u32) {
        self.expenses.push(amount);
    }

    pub fn total_expenses(&self) -> u32 {
        self.expenses.iter().sum()
    }

    pub fn remaining_allowance(&self) -> u32 {
        (self.days_traveled * self.daily_allowance) - self.total_expenses()
    }

    pub fn summary(&self) -> String {
        let mut summary = String::from("info");
        summary.push_str(&String::from("info"));
        summary.push_str(&String::from("info"));
        summary.push_str(&String::from("info"));
        summary.push_str(&String::from("info"));
        summary
    }
}
