extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct UsbMassStorage {
    device_id: u32,
    capacity: u64,
    partitions: Vec<String>,
}

impl UsbMassStorage {
    pub fn new(device_id: u32, capacity: u64) -> Self {
        UsbMassStorage {
            device_id,
            capacity,
            partitions: Vec::new(),
        }
    }

    pub fn add_partition(&mut self, partition_name: &str) {
        let name = String::from(partition_name);
        self.partitions.push(name);
    }

    pub fn get_device_id(&self) -> u32 {
        self.device_id
    }

    pub fn get_capacity(&self) -> u64 {
        self.capacity
    }

    pub fn list_partitions(&self) -> Vec<&str> {
        self.partitions.iter().map(|p| p.as_str()).collect()
    }

    pub fn remove_partition(&mut self, partition_name: &str) {
        if let Some(index) = self.partitions.iter().position(|p| p == partition_name) {
            self.partitions.remove(index);
        }
    }
}
