extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct SmartHomeTriggerSunset {
    sunset_time: u32,
    devices_to_trigger: Vec<String>,
    is_active: bool,
}

impl SmartHomeTriggerSunset {
    pub fn new(sunset_time: u32) -> Self {
        SmartHomeTriggerSunset {
            sunset_time,
            devices_to_trigger: Vec::new(),
            is_active: false,
        }
    }

    pub fn add_device(&mut self, device_name: &str) {
        self.devices_to_trigger.push(device_name.to_string());
    }

    pub fn remove_device(&mut self, device_name: &str) -> bool {
        if let Some(index) = self.devices_to_trigger.iter().position(|d| d == device_name) {
            self.devices_to_trigger.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_devices(&self) -> &[String] {
        &self.devices_to_trigger
    }

    pub fn activate(&mut self) {
        self.is_active = true;
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    pub fn is_triggered(&self, current_time: u32) -> bool {
        self.is_active && current_time >= self.sunset_time
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smart_home_trigger_sunset() {
        let mut trigger = SmartHomeTriggerSunset::new(1800); // 6 PM

        assert!(!trigger.is_active);
        assert!(trigger.get_devices().is_empty());

        trigger.add_device("Light");
        trigger.add_device("Thermostat");

        assert_eq!(trigger.get_devices(), &["Light", "Thermostat"]);

        assert!(trigger.remove_device("Light"));
        assert!(!trigger.remove_device("Fan"));

        assert_eq!(trigger.get_devices(), &["Thermostat"]);

        trigger.activate();
        assert!(trigger.is_active);

        assert!(!trigger.is_triggered(1700)); // 5 PM
        assert!(trigger.is_triggered(1800)); // 6 PM

        trigger.deactivate();
        assert!(!trigger.is_active);
    }
}
