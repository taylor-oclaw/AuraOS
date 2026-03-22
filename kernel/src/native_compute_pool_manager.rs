extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct NativeComputePoolManager {
    pool: Vec<ComputeResource>,
}

impl NativeComputePoolManager {
    pub fn new() -> Self {
        NativeComputePoolManager { pool: Vec::new() }
    }

    pub fn add_resource(&mut self, resource: ComputeResource) {
        self.pool.push(resource);
    }

    pub fn remove_resource(&mut self, index: usize) -> Option<ComputeResource> {
        if index < self.pool.len() {
            Some(self.pool.remove(index))
        } else {
            None
        }
    }

    pub fn get_resource(&self, index: usize) -> Option<&ComputeResource> {
        self.pool.get(index)
    }

    pub fn list_resources(&self) -> &[ComputeResource] {
        &self.pool
    }

    pub fn find_resource_by_id(&self, id: u32) -> Option<&ComputeResource> {
        self.pool.iter().find(|r| r.id == id)
    }
}

pub struct ComputeResource {
    pub id: u32,
    pub name: String,
    pub capacity: u64,
    pub usage: u64,
}

impl ComputeResource {
    pub fn new(id: u32, name: String, capacity: u64) -> Self {
        ComputeResource { id, name, capacity, usage: 0 }
    }

    pub fn allocate(&mut self, amount: u64) -> bool {
        if self.usage + amount <= self.capacity {
            self.usage += amount;
            true
        } else {
            false
        }
    }

    pub fn deallocate(&mut self, amount: u64) {
        if amount <= self.usage {
            self.usage -= amount;
        }
    }

    pub fn available_capacity(&self) -> u64 {
        self.capacity - self.usage
    }
}