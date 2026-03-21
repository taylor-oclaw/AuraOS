extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct SmartHomePoolControl {
    pool_temperature: u8,
    pump_status: bool,
    filter_status: bool,
    heater_status: bool,
    light_status: bool,
    water_level: u8,
}

impl SmartHomePoolControl {
    pub fn new() -> Self {
        SmartHomePoolControl {
            pool_temperature: 25, // Default temperature in Celsius
            pump_status: false,
            filter_status: false,
            heater_status: false,
            light_status: false,
            water_level: 80, // Default water level percentage
        }
    }

    pub fn set_pool_temperature(&mut self, temperature: u8) {
        if temperature >= 15 && temperature <= 40 {
            self.pool_temperature = temperature;
        }
    }

    pub fn get_pool_temperature(&self) -> u8 {
        self.pool_temperature
    }

    pub fn toggle_pump(&mut self) {
        self.pump_status = !self.pump_status;
    }

    pub fn is_pump_on(&self) -> bool {
        self.pump_status
    }

    pub fn toggle_filter(&mut self) {
        self.filter_status = !self.filter_status;
    }

    pub fn is_filter_on(&self) -> bool {
        self.filter_status
    }

    pub fn toggle_heater(&mut self) {
        if self.pool_temperature < 25 {
            self.heater_status = true;
        } else {
            self.heater_status = false;
        }
    }

    pub fn is_heater_on(&self) -> bool {
        self.heater_status
    }

    pub fn toggle_light(&mut self) {
        self.light_status = !self.light_status;
    }

    pub fn is_light_on(&self) -> bool {
        self.light_status
    }

    pub fn set_water_level(&mut self, level: u8) {
        if level >= 0 && level <= 100 {
            self.water_level = level;
        }
    }

    pub fn get_water_level(&self) -> u8 {
        self.water_level
    }

    pub fn status_report(&self) -> String {
        let mut report = String::from("Pool Status Report:\n");
        report.push_str(&format!("Temperature: {}°C\n", self.pool_temperature));
        report.push_str(&format!("Pump: {}\n", if self.pump_status { "On" } else { "Off" }));
        report.push_str(&format!("Filter: {}\n", if self.filter_status { "On" } else { "Off" }));
        report.push_str(&format!("Heater: {}\n", if self.heater_status { "On" } else { "Off" }));
        report.push_str(&format!("Light: {}\n", if self.light_status { "On" } else { "Off" }));
        report.push_str(&format!("Water Level: {}%\n", self.water_level));
        report
    }
}

#[no_mangle]
pub extern "C" fn smart_home_pool_control_init() -> *mut SmartHomePoolControl {
    Box::into_raw(Box::new(SmartHomePoolControl::new()))
}

#[no_mangle]
pub extern "C" fn smart_home_pool_control_destroy(ptr: *mut SmartHomePoolControl) {
    unsafe { drop(Box::from_raw(ptr)) }
}
