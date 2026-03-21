extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct CameraRelay {
    device_id: String,
    connected_devices: Vec<String>,
    buffer_size: usize,
    is_active: bool,
}

impl CameraRelay {
    pub fn new(device_id: &str, buffer_size: usize) -> Self {
        CameraRelay {
            device_id: String::from(device_id),
            connected_devices: Vec::new(),
            buffer_size,
            is_active: false,
        }
    }

    pub fn connect_device(&mut self, device_id: &str) -> bool {
        if !self.connected_devices.contains(&String::from(device_id)) {
            self.connected_devices.push(String::from(device_id));
            true
        } else {
            false
        }
    }

    pub fn disconnect_device(&mut self, device_id: &str) -> bool {
        let pos = self.connected_devices.iter().position(|x| x == device_id);
        if let Some(index) = pos {
            self.connected_devices.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_connected_devices(&self) -> Vec<String> {
        self.connected_devices.clone()
    }

    pub fn start_relay(&mut self) {
        self.is_active = true;
    }

    pub fn stop_relay(&mut self) {
        self.is_active = false;
    }

    pub fn is_relay_active(&self) -> bool {
        self.is_active
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_relay() {
        let mut relay = CameraRelay::new("device1", 1024);
        assert_eq!(relay.list_connected_devices().len(), 0);

        assert!(relay.connect_device("device2"));
        assert_eq!(relay.list_connected_devices().len(), 1);
        assert!(!relay.connect_device("device2")); // Should not add duplicate

        assert!(relay.disconnect_device("device2"));
        assert_eq!(relay.list_connected_devices().len(), 0);

        relay.start_relay();
        assert!(relay.is_relay_active());

        relay.stop_relay();
        assert!(!relay.is_relay_active());
    }
}
