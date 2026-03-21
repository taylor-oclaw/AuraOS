extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct ProfileWFHLunchPersonal {
    name: String,
    lunch_preferences: Vec<String>,
    work_from_home_days: u8,
    personal_goals: Vec<String>,
}

impl ProfileWFHLunchPersonal {
    pub fn new(name: &str) -> Self {
        ProfileWFHLunchPersonal {
            name: String::from(name),
            lunch_preferences: Vec::new(),
            work_from_home_days: 0,
            personal_goals: Vec::new(),
        }
    }

    pub fn add_lunch_preference(&mut self, preference: &str) {
        self.lunch_preferences.push(String::from(preference));
    }

    pub fn set_work_from_home_days(&mut self, days: u8) {
        self.work_from_home_days = days;
    }

    pub fn add_personal_goal(&mut self, goal: &str) {
        self.personal_goals.push(String::from(goal));
    }

    pub fn get_lunch_preferences(&self) -> &Vec<String> {
        &self.lunch_preferences
    }

    pub fn get_work_from_home_days(&self) -> u8 {
        self.work_from_home_days
    }

    pub fn get_personal_goals(&self) -> &Vec<String> {
        &self.personal_goals
    }
}
