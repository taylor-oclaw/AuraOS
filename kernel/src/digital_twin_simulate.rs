extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut twin = DigitalTwinSimulate::new("Device123".into(), 42);
    twin.update_temperature(30.5);
    twin.log_status();
    twin.reset_device();
    twin.set_power_state(true);
    twin.log_status();

    loop {}
}

pub struct DigitalTwinSimulate {
    device_id: String,
    temperature: f32,
    power_on: bool,
    status_log: Vec<String>,
}

impl DigitalTwinSimulate {
    pub fn new(device_id: String, initial_temperature: i32) -> Self {
        let mut twin = DigitalTwinSimulate {
            device_id,
            temperature: initial_temperature as f32,
            power_on: false,
            status_log: Vec::new(),
        };
        twin.log_status();
        twin
    }

    pub fn update_temperature(&mut self, new_temp: f32) {
        self.temperature = new_temp;
        self.log_status();
    }

    pub fn log_status(&mut self) {
        let status = String::from("info");
        self.status_log.push(status);
    }

    pub fn reset_device(&mut self) {
        self.temperature = 0.0;
        self.power_on = false;
        self.log_status();
    }

    pub fn set_power_state(&mut self, state: bool) {
        self.power_on = state;
        self.log_status();
    }
}
