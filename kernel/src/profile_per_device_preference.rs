extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut device_profile = DeviceProfile::new("AI-Device".into(), 8, 16);
    
    device_profile.add_sensor("Temperature".into());
    device_profile.add_sensor("Humidity".into());
    device_profile.add_sensor("Pressure".into());

    device_profile.update_sensor_value("Temperature", 25.0);
    device_profile.update_sensor_value("Humidity", 45.0);

    let temperature = device_profile.get_sensor_value("Temperature");
    if let Some(temp) = temperature {
        println!("Current Temperature: {}°C", temp);
    }

    loop {}
}

pub struct DeviceProfile {
    name: String,
    num_cores: u8,
    memory_gb: u16,
    sensors: Vec<Sensor>,
}

impl DeviceProfile {
    pub fn new(name: String, num_cores: u8, memory_gb: u16) -> Self {
        DeviceProfile {
            name,
            num_cores,
            memory_gb,
            sensors: Vec::new(),
        }
    }

    pub fn add_sensor(&mut self, sensor_name: String) {
        let sensor = Sensor {
            name: sensor_name,
            value: None,
        };
        self.sensors.push(sensor);
    }

    pub fn update_sensor_value(&mut self, sensor_name: &str, value: f32) {
        if let Some(sensor) = self.sensors.iter_mut().find(|s| s.name == sensor_name) {
            sensor.value = Some(value);
        }
    }

    pub fn get_sensor_value(&self, sensor_name: &str) -> Option<f32> {
        self.sensors
            .iter()
            .find(|s| s.name == sensor_name)
            .and_then(|s| s.value)
    }

    pub fn list_sensors(&self) -> Vec<&String> {
        self.sensors.iter().map(|s| &s.name).collect()
    }
}

struct Sensor {
    name: String,
    value: Option<f32>,
}
