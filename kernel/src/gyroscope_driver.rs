extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn gyroscope_driver_init() -> i32 {
    // Initialization logic for the gyroscope driver
    0
}

#[no_mangle]
pub extern "C" fn gyroscope_driver_exit() {
    // Cleanup logic for the gyroscope driver
}

pub struct GyroscopeDriver {
    sensor_id: u32,
    calibration_data: Vec<f32>,
    readings: Vec<(f32, f32, f32)>, // (x, y, z)
    status: String,
}

impl GyroscopeDriver {
    pub fn new(sensor_id: u32) -> Self {
        GyroscopeDriver {
            sensor_id,
            calibration_data: vec![0.0; 3],
            readings: Vec::new(),
            status: String::from("Initialized"),
        }
    }

    pub fn calibrate(&mut self, data: &[f32]) {
        if data.len() == 3 {
            self.calibration_data.copy_from_slice(data);
            self.status = String::from("Calibrated");
        } else {
            self.status = String::from("Calibration failed");
        }
    }

    pub fn read_sensor(&mut self, x: f32, y: f32, z: f32) -> bool {
        if self.calibration_data.iter().all(|&v| v != 0.0) {
            let calibrated_x = x - self.calibration_data[0];
            let calibrated_y = y - self.calibration_data[1];
            let calibrated_z = z - self.calibration_data[2];
            self.readings.push((calibrated_x, calibrated_y, calibrated_z));
            true
        } else {
            false
        }
    }

    pub fn get_latest_reading(&self) -> Option<(f32, f32, f32)> {
        self.readings.last().cloned()
    }

    pub fn get_status(&self) -> &str {
        &self.status
    }
}
