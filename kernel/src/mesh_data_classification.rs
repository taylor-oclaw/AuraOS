extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MeshDataClassification {
    data: Vec<u8>,
    labels: Vec<String>,
}

impl MeshDataClassification {
    pub fn new(data: Vec<u8>, labels: Vec<String>) -> Self {
        MeshDataClassification { data, labels }
    }

    pub fn add_data(&mut self, new_data: u8) {
        self.data.push(new_data);
    }

    pub fn add_label(&mut self, new_label: String) {
        self.labels.push(new_label);
    }

    pub fn classify(&self, index: usize) -> Option<&String> {
        if index < self.labels.len() {
            Some(&self.labels[index])
        } else {
            None
        }
    }

    pub fn get_data_length(&self) -> usize {
        self.data.len()
    }

    pub fn get_labels_length(&self) -> usize {
        self.labels.len()
    }
}
