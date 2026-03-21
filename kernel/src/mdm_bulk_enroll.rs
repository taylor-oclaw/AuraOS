extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

#[derive(Debug)]
pub struct MdmBulkEnroll {
    devices: Vec<String>,
    enrolled_count: usize,
}

impl MdmBulkEnroll {
    pub fn new() -> Self {
        MdmBulkEnroll {
            devices: Vec::new(),
            enrolled_count: 0,
        }
    }

    pub fn add_device(&mut self, device_id: &str) {
        if !self.devices.contains(&device_id.to_string()) {
            self.devices.push(device_id.to_string());
            self.enrolled_count += 1;
        }
    }

    pub fn remove_device(&mut self, device_id: &str) -> bool {
        if let Some(index) = self.devices.iter().position(|d| d == device_id) {
            self.devices.remove(index);
            self.enrolled_count -= 1;
            true
        } else {
            false
        }
    }

    pub fn get_device_count(&self) -> usize {
        self.devices.len()
    }

    pub fn list_devices(&self) -> Vec<String> {
        self.devices.clone()
    }

    pub fn is_device_enrolled(&self, device_id: &str) -> bool {
        self.devices.contains(&device_id.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mdm_bulk_enroll() {
        let mut enroll = MdmBulkEnroll::new();
        assert_eq!(enroll.get_device_count(), 0);
        assert!(!enroll.is_device_enrolled("device1"));

        enroll.add_device("device1");
        assert_eq!(enroll.get_device_count(), 1);
        assert!(enroll.is_device_enrolled("device1"));

        enroll.add_device("device2");
        assert_eq!(enroll.get_device_count(), 2);
        assert!(enroll.is_device_enrolled("device2"));

        let devices = enroll.list_devices();
        assert_eq!(devices, vec![String::from("device1"), String::from("device2")]);

        assert!(enroll.remove_device("device1"));
        assert_eq!(enroll.get_device_count(), 1);
        assert!(!enroll.is_device_enrolled("device1"));

        assert!(!enroll.remove_device("device3"));
        assert_eq!(enroll.get_device_count(), 1);
    }
}
