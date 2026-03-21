extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct MealPredictor {
    meal_times: Vec<u32>,
}

impl MealPredictor {
    pub fn new() -> Self {
        MealPredictor {
            meal_times: Vec::new(),
        }
    }

    pub fn add_meal_time(&mut self, time: u32) {
        self.meal_times.push(time);
    }

    pub fn get_meal_times(&self) -> &Vec<u32> {
        &self.meal_times
    }

    pub fn average_meal_time(&self) -> Option<u32> {
        if self.meal_times.is_empty() {
            None
        } else {
            let total: u32 = self.meal_times.iter().sum();
            Some(total / self.meal_times.len() as u32)
        }
    }

    pub fn earliest_meal_time(&self) -> Option<u32> {
        self.meal_times.iter().min().cloned()
    }

    pub fn latest_meal_time(&self) -> Option<u32> {
        self.meal_times.iter().max().cloned()
    }
}
