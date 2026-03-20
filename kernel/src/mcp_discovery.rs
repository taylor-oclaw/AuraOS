extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

mod mcp_discovery {
    use super::*;

    pub struct MCPDiscovery {
        devices: Vec<String>,
        last_scanned_time: u64,
    }

    impl MCPDiscovery {
        pub fn new() -> Self {
            MCPDiscovery {
                devices: Vec::new(),
                last_scanned_time: 0,
            }
        }

        pub fn scan(&mut self) {
            // Simulate scanning for devices
            let new_devices = vec![
                String::from("Device1"),
                String::from("Device2"),
                String::from("Device3"),
            ];
            self.devices.extend(new_devices);
            self.last_scanned_time = 1672531200; // Example timestamp
        }

        pub fn get_device_count(&self) -> usize {
            self.devices.len()
        }

        pub fn list_devices(&self) -> Vec<String> {
            self.devices.clone()
        }

        pub fn is_device_connected(&self, device_name: &str) -> bool {
            self.devices.contains(&String::from(device_name))
        }

        pub fn get_last_scan_time(&self) -> u64 {
            self.last_scanned_time
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mcp_discovery() {
        let mut discovery = mcp_discovery::MCPDiscovery::new();
        assert_eq!(discovery.get_device_count(), 0);
        assert!(!discovery.is_device_connected("Device1"));

        discovery.scan();
        assert_eq!(discovery.get_device_count(), 3);
        assert!(discovery.is_device_connected("Device1"));
        assert_eq!(discovery.list_devices(), vec![
            String::from("Device1"),
            String::from("Device2"),
            String::from("Device3"),
        ]);
        assert_eq!(discovery.get_last_scan_time(), 1672531200);
    }
}
