extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct SmartHomeTriggerPresence {
    device_id: String,
    sensors: Vec<String>,
    active: bool,
}

impl SmartHomeTriggerPresence {
    pub fn new(device_id: &str) -> Self {
        SmartHomeTriggerPresence {
            device_id: String::from(device_id),
            sensors: Vec::new(),
            active: false,
        }
    }

    pub fn add_sensor(&mut self, sensor_id: &str) {
        self.sensors.push(String::from(sensor_id));
    }

    pub fn remove_sensor(&mut self, sensor_id: &str) -> bool {
        if let Some(index) = self.sensors.iter().position(|s| s == sensor_id) {
            self.sensors.remove(index);
            true
        } else {
            false
        }
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn list_sensors(&self) -> Vec<String> {
        self.sensors.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smart_home_trigger_presence() {
        let mut trigger = SmartHomeTriggerPresence::new("device1");
        assert_eq!(trigger.device_id, "device1");
        assert!(!trigger.is_active());
        assert!(trigger.list_sensors().is_empty());

        trigger.add_sensor("sensor1");
        trigger.add_sensor("sensor2");
        assert_eq!(trigger.list_sensors(), vec![String::from("sensor1"), String::from("sensor2")]);

        assert!(trigger.remove_sensor("sensor1"));
        assert!(!trigger.remove_sensor("sensor3"));
        assert_eq!(trigger.list_sensors(), vec![String::from("sensor2")]);

        trigger.activate();
        assert!(trigger.is_active());

        trigger.deactivate();
        assert!(!trigger.is_active());
    }
}
