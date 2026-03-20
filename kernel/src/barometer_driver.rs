extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct BarometerDriver {
    pressure: u32,
    temperature: i32,
    altitude: f32,
    sea_level_pressure: u32,
    calibration_offset: i32,
}

impl BarometerDriver {
    pub fn new(pressure: u32, temperature: i32) -> Self {
        BarometerDriver {
            pressure,
            temperature,
            altitude: 0.0,
            sea_level_pressure: 101325,
            calibration_offset: 0,
        }
    }

    pub fn set_pressure(&mut self, pressure: u32) {
        self.pressure = pressure;
    }

    pub fn get_pressure(&self) -> u32 {
        self.pressure
    }

    pub fn set_temperature(&mut self, temperature: i32) {
        self.temperature = temperature;
    }

    pub fn get_temperature(&self) -> i32 {
        self.temperature
    }

    pub fn calculate_altitude(&mut self) {
        let pressure_ratio = (self.pressure as f32 / self.sea_level_pressure as f32).powf(1.0 / 5.256);
        self.altitude = 44330.0 * (1.0 - pressure_ratio);
    }

    pub fn get_altitude(&self) -> f32 {
        self.altitude
    }
}

#[no_mangle]
pub extern "C" fn barometer_init(pressure: u32, temperature: i32) -> *mut BarometerDriver {
    let driver = Box::new(BarometerDriver::new(pressure, temperature));
    Box::leak(driver)
}

#[no_mangle]
pub extern "C" fn barometer_set_pressure(driver: *mut BarometerDriver, pressure: u32) {
    unsafe { (*driver).set_pressure(pressure); }
}

#[no_mangle]
pub extern "C" fn barometer_get_pressure(driver: *const BarometerDriver) -> u32 {
    unsafe { (*driver).get_pressure() }
}

#[no_mangle]
pub extern "C" fn barometer_set_temperature(driver: *mut BarometerDriver, temperature: i32) {
    unsafe { (*driver).set_temperature(temperature); }
}

#[no_mangle]
pub extern "C" fn barometer_get_temperature(driver: *const BarometerDriver) -> i32 {
    unsafe { (*driver).get_temperature() }
}

#[no_mangle]
pub extern "C" fn barometer_calculate_altitude(driver: *mut BarometerDriver) {
    unsafe { (*driver).calculate_altitude(); }
}

#[no_mangle]
pub extern "C" fn barometer_get_altitude(driver: *const BarometerDriver) -> f32 {
    unsafe { (*driver).get_altitude() }
}
