extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

mod aura_control_center {
    use super::*;

    pub struct AuraControlCenter {
        devices: Vec<String>,
        active_device: Option<usize>,
    }

    impl AuraControlCenter {
        pub fn new() -> Self {
            AuraControlCenter {
                devices: Vec::new(),
                active_device: None,
            }
        }

        pub fn add_device(&mut self, device_name: &str) {
            self.devices.push(String::from(device_name));
        }

        pub fn remove_device(&mut self, device_name: &str) -> bool {
            if let Some(index) = self.devices.iter().position(|d| d == device_name) {
                self.devices.remove(index);
                if self.active_device == Some(index) {
                    self.active_device = None;
                }
                true
            } else {
                false
            }
        }

        pub fn set_active_device(&mut self, device_name: &str) -> bool {
            if let Some(index) = self.devices.iter().position(|d| d == device_name) {
                self.active_device = Some(index);
                true
            } else {
                false
            }
        }

        pub fn get_active_device(&self) -> Option<&String> {
            self.active_device.map(|index| &self.devices[index])
        }

        pub fn list_devices(&self) -> &[String] {
            &self.devices
        }
    }
}

#[cfg(test)]
mod tests {
    use super::aura_control_center::*;

    #[test]
    fn test_aura_control_center() {
        let mut acc = AuraControlCenter::new();
        assert_eq!(acc.list_devices().len(), 0);
        assert!(!acc.set_active_device("device1"));
        assert_eq!(acc.get_active_device(), None);

        acc.add_device("device1");
        assert_eq!(acc.list_devices().len(), 1);
        assert_eq!(acc.list_devices()[0], "device1");

        acc.add_device("device2");
        assert_eq!(acc.list_devices().len(), 2);
        assert_eq!(acc.list_devices()[1], "device2");

        assert!(acc.set_active_device("device1"));
        assert_eq!(acc.get_active_device(), Some(&String::from("device1")));

        assert!(acc.remove_device("device1"));
        assert_eq!(acc.list_devices().len(), 1);
        assert_eq!(acc.list_devices()[0], "device2");
        assert_eq!(acc.get_active_device(), None);

        assert!(!acc.remove_device("device3"));
        assert_eq!(acc.list_devices().len(), 1);
    }
}
