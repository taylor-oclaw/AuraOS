extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct DisplayConnector {
    name: String,
    resolution: Vec<u32>,
}

impl DisplayConnector {
    pub fn new(name: &str) -> Self {
        DisplayConnector {
            name: String::from(name),
            resolution: vec![],
        }
    }

    pub fn set_resolution(&mut self, width: u32, height: u32) {
        self.resolution = vec![width, height];
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_resolution(&self) -> Vec<u32> {
        self.resolution.clone()
    }

    pub fn is_connected(&self) -> bool {
        true // Replace with actual logic to check if the display connector is connected
    }
}

pub struct DisplayConnectorManager {
    connectors: Vec<DisplayConnector>,
}

impl DisplayConnectorManager {
    pub fn new() -> Self {
        DisplayConnectorManager { connectors: vec![] }
    }

    pub fn add_connector(&mut self, connector: DisplayConnector) {
        self.connectors.push(connector);
    }

    pub fn get_connectors(&self) -> Vec<&DisplayConnector> {
        self.connectors.iter().collect()
    }
}