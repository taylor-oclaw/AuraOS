extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn smart_home_fridge_monitor_init() {
    // Initialization logic for the module
}

pub extern "C" fn smart_home_fridge_monitor_exit() {
    // Cleanup logic for the module
}

pub struct SmartHomeFridgeMonitor {
    items: Vec<String>,
    temperature: i32,
    power_consumption: u32,
    last_checked: String,
}

impl SmartHomeFridgeMonitor {
    pub fn new(initial_items: Vec<String>, initial_temperature: i32, initial_power_consumption: u32) -> Self {
        SmartHomeFridgeMonitor {
            items: initial_items,
            temperature: initial_temperature,
            power_consumption: initial_power_consumption,
            last_checked: String::from("Not checked yet"),
        }
    }

    pub fn add_item(&mut self, item: String) {
        self.items.push(item);
    }

    pub fn remove_item(&mut self, item: &str) -> bool {
        if let Some(index) = self.items.iter().position(|x| x == item) {
            self.items.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_items(&self) -> &Vec<String> {
        &self.items
    }

    pub fn update_temperature(&mut self, new_temperature: i32) {
        self.temperature = new_temperature;
        self.last_checked = String::from("Updated");
    }

    pub fn get_temperature(&self) -> i32 {
        self.temperature
    }

    pub fn update_power_consumption(&mut self, new_power_consumption: u32) {
        self.power_consumption = new_power_consumption;
        self.last_checked = String::from("Updated");
    }

    pub fn get_power_consumption(&self) -> u32 {
        self.power_consumption
    }

    pub fn check_status(&mut self, status: &str) {
        self.last_checked = String::from(status);
    }

    pub fn get_last_checked(&self) -> &String {
        &self.last_checked
    }
}
