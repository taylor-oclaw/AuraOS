extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct DriverPortLint {
    port: u16,
}

impl DriverPortLint {
    pub fn new(port: u16) -> Self {
        DriverPortLint { port }
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }

    pub fn set_port(&mut self, port: u16) {
        self.port = port;
    }

    pub fn is_valid_port(&self) -> bool {
        if self.port >= 0x3F8 && self.port <= 0x3FF {
            true
        } else {
            false
        }
    }

    pub fn get_driver_name(&self) -> String {
        match self.port {
            0x3F8 => "COM1".to_string(),
            0x2F8 => "COM2".to_string(),
            0x3E8 => "COM3".to_string(),
            0x2E8 => "COM4".to_string(),
            _ => format!("Unknown port: {}", self.port),
        }
    }

    pub fn get_driver_info(&self) -> String {
        let mut info = String::new();
        info.push_str("Driver name: ");
        info.push_str(self.get_driver_name().as_str());
        info.push_str("\n");
        info.push_str("Port address: 0x");
        info.push_str(&format!("{:04X}", self.port));
        info
    }
}