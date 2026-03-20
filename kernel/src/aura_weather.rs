extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AuraWeather {
    temperature: i32,
    humidity: u8,
    wind_speed: f32,
    weather_description: String,
    forecast: Vec<String>,
}

impl AuraWeather {
    pub fn new(temperature: i32, humidity: u8, wind_speed: f32, description: &str) -> Self {
        AuraWeather {
            temperature,
            humidity,
            wind_speed,
            weather_description: String::from(description),
            forecast: Vec::new(),
        }
    }

    pub fn update_temperature(&mut self, new_temp: i32) {
        self.temperature = new_temp;
    }

    pub fn update_humidity(&mut self, new_humidity: u8) {
        self.humidity = new_humidity;
    }

    pub fn update_wind_speed(&mut self, new_wind_speed: f32) {
        self.wind_speed = new_wind_speed;
    }

    pub fn update_description(&mut self, new_description: &str) {
        self.weather_description = String::from(new_description);
    }

    pub fn add_forecast(&mut self, forecast_item: &str) {
        self.forecast.push(String::from(forecast_item));
    }

    pub fn get_temperature(&self) -> i32 {
        self.temperature
    }

    pub fn get_humidity(&self) -> u8 {
        self.humidity
    }

    pub fn get_wind_speed(&self) -> f32 {
        self.wind_speed
    }

    pub fn get_description(&self) -> &str {
        &self.weather_description
    }

    pub fn get_forecast(&self) -> &[String] {
        &self.forecast
    }
}
