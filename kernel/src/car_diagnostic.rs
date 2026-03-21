extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut diagnostic = CarDiagnostic::new();
    diagnostic.connect_sensor("Engine Temp");
    diagnostic.connect_sensor("Oil Pressure");
    diagnostic.connect_sensor("Battery Voltage");

    if diagnostic.check_engine_temp() {
        println!("Engine temperature is within normal range.");
    } else {
        println!("Warning: Engine temperature is too high!");
    }

    if diagnostic.check_oil_pressure() {
        println!("Oil pressure is sufficient.");
    } else {
        println!("Warning: Oil pressure is low!");
    }

    if diagnostic.check_battery_voltage() {
        println!("Battery voltage is adequate.");
    } else {
        println!("Warning: Battery voltage is too low!");
    }
}

pub struct CarDiagnostic {
    sensors: Vec<String>,
    engine_temp: i32,
    oil_pressure: i32,
    battery_voltage: i32,
}

impl CarDiagnostic {
    pub fn new() -> Self {
        CarDiagnostic {
            sensors: Vec::new(),
            engine_temp: 0,
            oil_pressure: 0,
            battery_voltage: 0,
        }
    }

    pub fn connect_sensor(&mut self, sensor_name: &str) {
        self.sensors.push(String::from(sensor_name));
    }

    pub fn update_engine_temp(&mut self, temp: i32) {
        self.engine_temp = temp;
    }

    pub fn update_oil_pressure(&mut self, pressure: i32) {
        self.oil_pressure = pressure;
    }

    pub fn update_battery_voltage(&mut self, voltage: i32) {
        self.battery_voltage = voltage;
    }

    pub fn check_engine_temp(&self) -> bool {
        self.engine_temp < 100 // Assuming normal range is below 100 degrees Celsius
    }

    pub fn check_oil_pressure(&self) -> bool {
        self.oil_pressure > 20 // Assuming sufficient pressure is above 20 PSI
    }

    pub fn check_battery_voltage(&self) -> bool {
        self.battery_voltage > 12 // Assuming adequate voltage is above 12 volts
    }
}
