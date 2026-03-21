extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let weather_app = MiniAppWeatherGlance::new();
    weather_app.display_weather_info();
    loop {}
}

pub struct MiniAppWeatherGlance {
    location: String,
    temperature: i32,
    humidity: u8,
    wind_speed: f32,
    conditions: Vec<String>,
}

impl MiniAppWeatherGlance {
    pub fn new() -> Self {
        MiniAppWeatherGlance {
            location: String::from("New York"),
            temperature: 15, // in Celsius
            humidity: 78,
            wind_speed: 5.2, // in km/h
            conditions: vec![String::from("Sunny"), String::from("Cloudy")],
        }
    }

    pub fn update_location(&mut self, new_location: &str) {
        self.location = String::from(new_location);
    }

    pub fn get_temperature(&self) -> i32 {
        self.temperature
    }

    pub fn set_temperature(&mut self, temp: i32) {
        self.temperature = temp;
    }

    pub fn get_humidity(&self) -> u8 {
        self.humidity
    }

    pub fn set_humidity(&mut self, humidity: u8) {
        self.humidity = humidity;
    }

    pub fn get_wind_speed(&self) -> f32 {
        self.wind_speed
    }

    pub fn set_wind_speed(&mut self, wind_speed: f32) {
        self.wind_speed = wind_speed;
    }

    pub fn add_condition(&mut self, condition: &str) {
        self.conditions.push(String::from(condition));
    }

    pub fn remove_condition(&mut self, condition: &str) {
        self.conditions.retain(|c| c != condition);
    }

    pub fn display_weather_info(&self) {
        // Simulate displaying weather info
        println!("Weather in {}: {}°C, Humidity: {}%, Wind Speed: {} km/h", 
                 self.location, self.temperature, self.humidity, self.wind_speed);
        for condition in &self.conditions {
            println!("Condition: {}", condition);
        }
    }
}
