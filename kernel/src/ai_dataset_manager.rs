extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

struct AIDatasetManager {
    datasets: Vec<String>,
}

impl AIDatasetManager {
    pub fn new() -> Self {
        AIDatasetManager {
            datasets: Vec::new(),
        }
    }

    pub fn add_dataset(&mut self, name: &str) {
        self.datasets.push(String::from(name));
    }

    pub fn remove_dataset(&mut self, name: &str) {
        if let Some(index) = self.datasets.iter().position(|d| d == name) {
            self.datasets.remove(index);
        }
    }

    pub fn list_datasets(&self) -> Vec<String> {
        self.datasets.clone()
    }

    pub fn dataset_exists(&self, name: &str) -> bool {
        self.datasets.contains(&String::from(name))
    }

    pub fn get_dataset_count(&self) -> usize {
        self.datasets.len()
    }
}
