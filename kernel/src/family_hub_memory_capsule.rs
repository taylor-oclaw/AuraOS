extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut capsule = FamilyHubMemoryCapsule::new();
    capsule.store_data("Hello, Kernel!".to_string());
    capsule.update_data(0, "Updated Data".to_string()).unwrap();
    capsule.delete_data(0).unwrap();

    loop {}
}

pub struct FamilyHubMemoryCapsule {
    data_store: Vec<String>,
}

impl FamilyHubMemoryCapsule {
    pub fn new() -> Self {
        FamilyHubMemoryCapsule {
            data_store: Vec::new(),
        }
    }

    pub fn store_data(&mut self, data: String) -> usize {
        let index = self.data_store.len();
        self.data_store.push(data);
        index
    }

    pub fn retrieve_data(&self, index: usize) -> Option<&String> {
        self.data_store.get(index)
    }

    pub fn update_data(&mut self, index: usize, data: String) -> Result<(), &'static str> {
        if let Some(entry) = self.data_store.get_mut(index) {
            *entry = data;
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }

    pub fn delete_data(&mut self, index: usize) -> Result<(), &'static str> {
        if index < self.data_store.len() {
            self.data_store.remove(index);
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }

    pub fn retrieve_all_data(&self) -> Vec<&String> {
        self.data_store.iter().collect()
    }
}
