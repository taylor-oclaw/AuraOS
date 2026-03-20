extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

mod cxl_handler {
    use super::*;

    pub struct CxlHandler {
        devices: Vec<String>,
        active_device: Option<usize>,
    }

    impl CxlHandler {
        pub fn new() -> Self {
            CxlHandler {
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
                true
            } else {
                false
            }
        }

        pub fn list_devices(&self) -> Vec<String> {
            self.devices.clone()
        }

        pub fn set_active_device(&mut self, device_name: &str) -> bool {
            if let Some(index) = self.devices.iter().position(|d| d == device_name) {
                self.active_device = Some(index);
                true
            } else {
                false
            }
        }

        pub fn get_active_device(&self) -> Option<String> {
            self.active_device.map(|index| self.devices[index].clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::cxl_handler::*;

    #[test]
    fn test_cxl_handler() {
        let mut handler = CxlHandler::new();
        assert_eq!(handler.list_devices(), Vec::<String>::new());

        handler.add_device("device1");
        handler.add_device("device2");
        assert_eq!(handler.list_devices(), vec![String::from("device1"), String::from("device2")]);

        assert!(handler.set_active_device("device1"));
        assert_eq!(handler.get_active_device(), Some(String::from("device1")));

        assert!(handler.remove_device("device1"));
        assert_eq!(handler.list_devices(), vec![String::from("device2")]);
        assert_eq!(handler.get_active_device(), None);
    }
}
