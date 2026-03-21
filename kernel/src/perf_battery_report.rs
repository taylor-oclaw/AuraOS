extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct BatteryReport {
    battery_level: u8,
    charging_status: bool,
    voltage: u16,
    current: i16,
    temperature: i16,
}

impl BatteryReport {
    pub fn new(battery_level: u8, charging_status: bool, voltage: u16, current: i16, temperature: i16) -> Self {
        BatteryReport {
            battery_level,
            charging_status,
            voltage,
            current,
            temperature,
        }
    }

    pub fn get_battery_level(&self) -> u8 {
        self.battery_level
    }

    pub fn is_charging(&self) -> bool {
        self.charging_status
    }

    pub fn get_voltage(&self) -> u16 {
        self.voltage
    }

    pub fn get_current(&self) -> i16 {
        self.current
    }

    pub fn get_temperature(&self) -> i16 {
        self.temperature
    }
}

pub extern "C" fn create_battery_report(battery_level: u8, charging_status: bool, voltage: u16, current: i16, temperature: i16) -> *mut BatteryReport {
    let report = Box::new(BatteryReport::new(battery_level, charging_status, voltage, current, temperature));
    Box::into_raw(report)
}

pub extern "C" fn destroy_battery_report(report: *mut BatteryReport) {
    if !report.is_null() {
        unsafe { drop(Box::from_raw(report)); }
    }
}
