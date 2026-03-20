extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn alexa_bridge_init() {
    // Initialization logic for the module
}

pub extern "C" fn alexa_bridge_exit() {
    // Cleanup logic for the module
}

pub struct AlexaBridge {
    devices: Vec<String>,
    commands: Vec<String>,
}

impl AlexaBridge {
    pub fn new() -> Self {
        AlexaBridge {
            devices: Vec::new(),
            commands: Vec::new(),
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

    pub fn add_command(&mut self, command: &str) {
        self.commands.push(String::from(command));
    }

    pub fn execute_command(&self, command: &str) -> bool {
        self.commands.contains(&String::from(command))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alexa_bridge() {
        let mut bridge = AlexaBridge::new();
        assert_eq!(bridge.list_devices().len(), 0);

        bridge.add_device("light");
        assert_eq!(bridge.list_devices().len(), 1);
        assert_eq!(bridge.list_devices()[0], "light");

        assert!(bridge.remove_device("light"));
        assert_eq!(bridge.list_devices().len(), 0);

        bridge.add_command("turn on light");
        assert!(bridge.execute_command("turn on light"));

        assert!(!bridge.execute_command("turn off light"));
    }
}
