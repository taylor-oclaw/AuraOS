extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct SmartHomeHVACControl {
    temperature: i32,
    humidity: u8,
    mode: String,
    fan_speed: String,
    is_on: bool,
}

impl SmartHomeHVACControl {
    pub fn new(initial_temperature: i32, initial_humidity: u8) -> Self {
        SmartHomeHVACControl {
            temperature: initial_temperature,
            humidity: initial_humidity,
            mode: String::from("AUTO"),
            fan_speed: String::from("MEDIUM"),
            is_on: false,
        }
    }

    pub fn turn_on(&mut self) {
        self.is_on = true;
    }

    pub fn turn_off(&mut self) {
        self.is_on = false;
    }

    pub fn set_temperature(&mut self, temp: i32) {
        if self.is_on && (temp >= 16 && temp <= 30) {
            self.temperature = temp;
        }
    }

    pub fn set_humidity(&mut self, humid: u8) {
        if self.is_on && (humid >= 30 && humid <= 70) {
            self.humidity = humid;
        }
    }

    pub fn set_mode(&mut self, mode: &str) {
        let valid_modes = ["AUTO", "COOL", "HEAT", "FAN"];
        if valid_modes.contains(&mode) {
            self.mode = String::from(mode);
        }
    }

    pub fn set_fan_speed(&mut self, speed: &str) {
        let valid_speeds = ["LOW", "MEDIUM", "HIGH"];
        if valid_speeds.contains(&speed) {
            self.fan_speed = String::from(speed);
        }
    }

    pub fn status(&self) -> String {
        format!(
            "HVAC Status: {} - Temp: {}°C, Humidity: {}%, Mode: {}, Fan Speed: {}",
            if self.is_on { "ON" } else { "OFF" },
            self.temperature,
            self.humidity,
            self.mode,
            self.fan_speed
        )
    }
}
