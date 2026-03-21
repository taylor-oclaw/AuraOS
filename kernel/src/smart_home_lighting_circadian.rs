extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct SmartHomeLightingCircadian {
    light_level: u8,
    color_temperature: u16,
    brightness: f32,
    is_on: bool,
    schedule: Vec<(u8, u8, String)>, // (hour, minute, action)
}

impl SmartHomeLightingCircadian {
    pub fn new() -> Self {
        SmartHomeLightingCircadian {
            light_level: 50,
            color_temperature: 6500, // Default to daylight
            brightness: 1.0,
            is_on: false,
            schedule: Vec::new(),
        }
    }

    pub fn turn_on(&mut self) {
        self.is_on = true;
    }

    pub fn turn_off(&mut self) {
        self.is_on = false;
    }

    pub fn set_brightness(&mut self, brightness: f32) {
        if brightness >= 0.0 && brightness <= 1.0 {
            self.brightness = brightness;
        }
    }

    pub fn set_color_temperature(&mut self, color_temperature: u16) {
        // Typical range for color temperature is 2700K to 6500K
        if color_temperature >= 2700 && color_temperature <= 6500 {
            self.color_temperature = color_temperature;
        }
    }

    pub fn add_schedule(&mut self, hour: u8, minute: u8, action: String) {
        self.schedule.push((hour, minute, action));
    }

    pub fn execute_scheduled_actions(&mut self, current_hour: u8, current_minute: u8) {
        for (hour, minute, action) in &self.schedule {
            if *hour == current_hour && *minute == current_minute {
                match action.as_str() {
                    "turn_on" => self.turn_on(),
                    "turn_off" => self.turn_off(),
                    "increase_brightness" => self.set_brightness(self.brightness + 0.1),
                    "decrease_brightness" => self.set_brightness(self.brightness - 0.1),
                    "set_daylight" => self.set_color_temperature(6500),
                    "set_warmer" => self.set_color_temperature(self.color_temperature + 500),
                    "set_cooler" => self.set_color_temperature(self.color_temperature - 500),
                    _ => {}
                }
            }
        }
    }

    pub fn get_status(&self) -> String {
        let status = format!(
            "Light is {} with brightness {:.2} and color temperature {}K",
            if self.is_on { "on" } else { "off" },
            self.brightness,
            self.color_temperature
        );
        status
    }
}
