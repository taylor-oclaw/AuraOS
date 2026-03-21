extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ContextWeatherAware {
    location: String,
    temperature: i32,
    humidity: u8,
    weather_description: String,
    forecast_days: Vec<String>,
}

impl ContextWeatherAware {
    pub fn new(location: &str, temperature: i32, humidity: u8, description: &str) -> Self {
        ContextWeatherAware {
            location: String::from(location),
            temperature,
            humidity,
            weather_description: String::from(description),
            forecast_days: Vec::new(),
        }
    }

    pub fn update_temperature(&mut self, new_temp: i32) {
        self.temperature = new_temp;
    }

    pub fn update_humidity(&mut self, new_humidity: u8) {
        if new_humidity <= 100 {
            self.humidity = new_humidity;
        }
    }

    pub fn set_weather_description(&mut self, description: &str) {
        self.weather_description = String::from(description);
    }

    pub fn add_forecast_day(&mut self, day_description: &str) {
        self.forecast_days.push(String::from(day_description));
    }

    pub fn get_summary(&self) -> String {
        let mut summary = String::from("info");
        summary += &String::from("info");
        summary += &String::from("info");
        summary += &String::from("info");
        if !self.forecast_days.is_empty() {
            summary += "Forecast:\n";
            for (i, day) in self.forecast_days.iter().enumerate() {
                summary += &String::from("info");
            }
        }
        summary
    }
}
