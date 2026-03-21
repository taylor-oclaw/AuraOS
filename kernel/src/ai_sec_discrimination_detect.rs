extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct AIDiscriminationDetect {
    data: Vec<u8>,
    labels: Vec<String>,
}

impl AIDiscriminationDetect {
    pub fn new(data: Vec<u8>, labels: Vec<String>) -> Self {
        AIDiscriminationDetect { data, labels }
    }

    pub fn add_data(&mut self, data: u8) {
        self.data.push(data);
    }

    pub fn remove_data(&mut self, index: usize) -> Option<u8> {
        if index < self.data.len() {
            Some(self.data.remove(index))
        } else {
            None
        }
    }

    pub fn add_label(&mut self, label: String) {
        self.labels.push(label);
    }

    pub fn remove_label(&mut self, index: usize) -> Option<String> {
        if index < self.labels.len() {
            Some(self.labels.remove(index))
        } else {
            None
        }
    }

    pub fn get_data_len(&self) -> usize {
        self.data.len()
    }

    pub fn get_labels_len(&self) -> usize {
        self.labels.len()
    }
}
