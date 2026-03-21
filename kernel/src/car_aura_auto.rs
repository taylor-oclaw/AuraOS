extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn car_aura_auto_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn car_aura_auto_exit() {
    // Cleanup logic for the module
}

pub struct CarAuraAuto {
    status: String,
    lights_on: bool,
    temperature: f32,
    speed: u32,
    passengers: Vec<String>,
}

impl CarAuraAuto {
    pub fn new() -> Self {
        CarAuraAuto {
            status: String::from("Initialized"),
            lights_on: false,
            temperature: 20.0,
            speed: 0,
            passengers: Vec::new(),
        }
    }

    pub fn turn_lights_on(&mut self) {
        self.lights_on = true;
        self.status = String::from("Lights On");
    }

    pub fn turn_lights_off(&mut self) {
        self.lights_on = false;
        self.status = String::from("Lights Off");
    }

    pub fn set_temperature(&mut self, temp: f32) {
        if temp >= 15.0 && temp <= 30.0 {
            self.temperature = temp;
            self.status = format!("Temperature Set to {:.1}°C", self.temperature);
        } else {
            self.status = String::from("Invalid Temperature");
        }
    }

    pub fn set_speed(&mut self, speed: u32) {
        if speed <= 120 {
            self.speed = speed;
            self.status = format!("Speed Set to {} km/h", self.speed);
        } else {
            self.status = String::from("Invalid Speed");
        }
    }

    pub fn add_passenger(&mut self, name: &str) {
        if self.passengers.len() < 5 {
            self.passengers.push(String::from(name));
            self.status = format!("Passenger {} Added", name);
        } else {
            self.status = String::from("Car is Full");
        }
    }

    pub fn get_status(&self) -> &str {
        &self.status
    }

    pub fn get_lights_status(&self) -> bool {
        self.lights_on
    }

    pub fn get_temperature(&self) -> f32 {
        self.temperature
    }

    pub fn get_speed(&self) -> u32 {
        self.speed
    }

    pub fn get_passengers(&self) -> &Vec<String> {
        &self.passengers
    }
}
