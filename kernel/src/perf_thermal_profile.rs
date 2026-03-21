extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct PerfThermalProfile {
    name: String,
    temperature: u32,
    power_consumption: u32,
    performance_metrics: Vec<u32>,
    thermal_sensors: Vec<String>,
}

impl PerfThermalProfile {
    pub fn new(name: &str, temperature: u32, power_consumption: u32) -> Self {
        PerfThermalProfile {
            name: String::from(name),
            temperature,
            power_consumption,
            performance_metrics: Vec::new(),
            thermal_sensors: Vec::new(),
        }
    }

    pub fn add_performance_metric(&mut self, metric: u32) {
        self.performance_metrics.push(metric);
    }

    pub fn add_thermal_sensor(&mut self, sensor_name: &str) {
        self.thermal_sensors.push(String::from(sensor_name));
    }

    pub fn get_average_temperature(&self) -> u32 {
        if self.temperature == 0 || self.thermal_sensors.is_empty() {
            return 0;
        }
        self.temperature / self.thermal_sensors.len() as u32
    }

    pub fn get_total_power_consumption(&self) -> u32 {
        self.power_consumption
    }

    pub fn get_performance_metrics(&self) -> &Vec<u32> {
        &self.performance_metrics
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perf_thermal_profile() {
        let mut profile = PerfThermalProfile::new("TestProfile", 75, 100);
        profile.add_performance_metric(80);
        profile.add_performance_metric(90);
        profile.add_thermal_sensor("sensor1");
        profile.add_thermal_sensor("sensor2");

        assert_eq!(profile.get_average_temperature(), 37);
        assert_eq!(profile.get_total_power_consumption(), 100);
        assert_eq!(profile.get_performance_metrics(), &vec![80, 90]);
    }
}
