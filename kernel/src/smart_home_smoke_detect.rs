extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct SmartHomeSmokeDetect {
    sensor_data: Vec<u8>,
    is_active: bool,
    threshold: u8,
    alerts: Vec<String>,
}

impl SmartHomeSmokeDetect {
    pub fn new(threshold: u8) -> Self {
        SmartHomeSmokeDetect {
            sensor_data: Vec::new(),
            is_active: true,
            threshold,
            alerts: Vec::new(),
        }
    }

    pub fn add_sensor_data(&mut self, data: u8) {
        if self.is_active {
            self.sensor_data.push(data);
        }
    }

    pub fn check_smoke_level(&self) -> bool {
        for &data in &self.sensor_data {
            if data > self.threshold {
                return true;
            }
        }
        false
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    pub fn activate(&mut self) {
        self.is_active = true;
    }

    pub fn get_alerts(&self) -> &Vec<String> {
        &self.alerts
    }

    pub fn clear_alerts(&mut self) {
        self.alerts.clear();
    }
}
