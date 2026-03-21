extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn smart_home_pet_feeder_init() {
    // Initialization logic for the pet feeder module
}

#[no_mangle]
pub extern "C" fn smart_home_pet_feeder_exit() {
    // Cleanup logic for the pet feeder module
}

pub struct SmartHomePetFeeder {
    food_amount: u32,
    feeding_times: Vec<u32>,
    last_feed_time: u32,
    pet_name: String,
    is_active: bool,
}

impl SmartHomePetFeeder {
    pub fn new(food_amount: u32, feeding_times: Vec<u32>, pet_name: &str) -> Self {
        SmartHomePetFeeder {
            food_amount,
            feeding_times,
            last_feed_time: 0,
            pet_name: String::from(pet_name),
            is_active: true,
        }
    }

    pub fn feed(&mut self, current_time: u32) {
        if self.is_active && self.should_feed(current_time) {
            self.food_amount -= 1;
            self.last_feed_time = current_time;
            // Logic to dispense food
        }
    }

    pub fn should_feed(&self, current_time: u32) -> bool {
        for &time in &self.feeding_times {
            if time == current_time && self.last_feed_time != current_time {
                return true;
            }
        }
        false
    }

    pub fn add_feeding_time(&mut self, time: u32) {
        if !self.feeding_times.contains(&time) {
            self.feeding_times.push(time);
        }
    }

    pub fn remove_feeding_time(&mut self, time: u32) {
        self.feeding_times.retain(|&t| t != time);
    }

    pub fn get_status(&self) -> String {
        let status = if self.is_active {
            "Active"
        } else {
            "Inactive"
        };
        format!("Pet Feeder for {} - Status: {}, Food Amount: {}", self.pet_name, status, self.food_amount)
    }
}
