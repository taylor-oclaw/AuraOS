extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct BatteryManager {
    battery_level: u8,
    charging_status: bool,
    capacity: u16,
    voltage: f32,
    temperature: i16,
}

impl BatteryManager {
    pub fn new(level: u8, charging: bool, cap: u16, volt: f32, temp: i16) -> Self {
        BatteryManager {
            battery_level: level,
            charging_status: charging,
            capacity: cap,
            voltage: volt,
            temperature: temp,
        }
    }

    pub fn get_battery_level(&self) -> u8 {
        self.battery_level
    }

    pub fn is_charging(&self) -> bool {
        self.charging_status
    }

    pub fn get_capacity(&self) -> u16 {
        self.capacity
    }

    pub fn get_voltage(&self) -> f32 {
        self.voltage
    }

    pub fn get_temperature(&self) -> i16 {
        self.temperature
    }

    pub fn update_battery_level(&mut self, new_level: u8) {
        if new_level <= 100 {
            self.battery_level = new_level;
        }
    }

    pub fn toggle_charging_status(&mut self) {
        self.charging_status = !self.charging_status;
    }

    pub fn set_capacity(&mut self, new_cap: u16) {
        if new_cap > 0 && new_cap <= 10000 {
            self.capacity = new_cap;
        }
    }

    pub fn update_voltage(&mut self, new_volt: f32) {
        if new_volt > 0.0 {
            self.voltage = new_volt;
        }
    }

    pub fn update_temperature(&mut self, new_temp: i16) {
        if new_temp >= -50 && new_temp <= 100 {
            self.temperature = new_temp;
        }
    }

    pub fn get_status_report(&self) -> String {
        let mut report = String::from("Battery Status Report:\n");
        report.push_str(&String::from("info"));
        report.push_str(&String::from("info"));
        report.push_str(&String::from("info"));
        report.push_str(&String::from("info"));
        report.push_str(&String::from("info"));
        report
    }
}
