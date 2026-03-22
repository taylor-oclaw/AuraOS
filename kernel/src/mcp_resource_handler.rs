extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let handler = MCPResourceHandler::new();
    handler.initialize_resources();
    handler.allocate_resource("CPU");
    handler.allocate_resource("GPU");
    handler.deallocate_resource("CPU");
    handler.list_resources();
}

pub struct MCPResourceHandler {
    resources: Vec<String>,
}

impl MCPResourceHandler {
    pub fn new() -> Self {
        MCPResourceHandler {
            resources: Vec::new(),
        }
    }

    pub fn initialize_resources(&mut self) {
        // Initialize with default resources
        self.resources.push(String::from("CPU"));
        self.resources.push(String::from("GPU"));
        self.resources.push(String::from("RAM"));
        self.resources.push(String::from("Storage"));
        self.resources.push(String::from("Network"));
    }

    pub fn allocate_resource(&mut self, resource: &str) {
        if !self.resources.contains(&String::from(resource)) {
            self.resources.push(String::from(resource));
            println!("Resource {} allocated.", resource);
        } else {
            println!("Resource {} is already allocated.", resource);
        }
    }

    pub fn deallocate_resource(&mut self, resource: &str) {
        if let Some(index) = self.resources.iter().position(|r| r == resource) {
            self.resources.remove(index);
            println!("Resource {} deallocated.", resource);
        } else {
            println!("Resource {} is not allocated.", resource);
        }
    }

    pub fn list_resources(&self) {
        println!("Current resources:");
        for resource in &self.resources {
            println!("{}", resource);
        }
    }

    pub fn check_resource_availability(&self, resource: &str) -> bool {
        self.resources.contains(&String::from(resource))
    }
}