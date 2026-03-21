extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut rel_boundary = RelBoundaryRespect::new();
    rel_boundary.add_resource("CPU");
    rel_boundary.add_resource("Memory");
    rel_boundary.add_resource("Disk");
    rel_boundary.add_resource("Network");
    rel_boundary.add_resource("GPU");

    rel_boundary.allocate("CPU", 10);
    rel_boundary.allocate("Memory", 512);
    rel_boundary.allocate("Disk", 100);
    rel_boundary.allocate("Network", 10);
    rel_boundary.allocate("GPU", 4);

    loop {}
}

pub struct RelBoundaryRespect {
    resources: Vec<(String, u32)>,
}

impl RelBoundaryRespect {
    pub fn new() -> Self {
        RelBoundaryRespect {
            resources: Vec::new(),
        }
    }

    pub fn add_resource(&mut self, name: &str, capacity: u32) {
        self.resources.push((String::from(name), capacity));
    }

    pub fn allocate(&mut self, resource_name: &str, amount: u32) -> bool {
        for (name, ref mut capacity) in self.resources.iter_mut() {
            if name == resource_name && *capacity >= amount {
                *capacity -= amount;
                return true;
            }
        }
        false
    }

    pub fn deallocate(&mut self, resource_name: &str, amount: u32) -> bool {
        for (name, ref mut capacity) in self.resources.iter_mut() {
            if name == resource_name {
                *capacity += amount;
                return true;
            }
        }
        false
    }

    pub fn get_capacity(&self, resource_name: &str) -> Option<u32> {
        for (name, capacity) in self.resources.iter() {
            if name == resource_name {
                return Some(*capacity);
            }
        }
        None
    }

    pub fn list_resources(&self) -> Vec<(String, u32)> {
        self.resources.clone()
    }
}
