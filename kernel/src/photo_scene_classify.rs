extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct PhotoSceneClassifier {
    model: Vec<u8>,
    labels: Vec<String>,
}

impl PhotoSceneClassifier {
    pub fn new(model_data: Vec<u8>, label_data: Vec<&str>) -> Self {
        let labels = label_data.into_iter().map(|s| s.to_string()).collect();
        PhotoSceneClassifier {
            model: model_data,
            labels,
        }
    }

    pub fn classify(&self, image_data: &[u8]) -> Option<String> {
        // Placeholder for actual classification logic
        if image_data.len() > 0 {
            Some(self.labels[0].clone())
        } else {
            None
        }
    }

    pub fn update_model(&mut self, new_model_data: Vec<u8>) {
        self.model = new_model_data;
    }

    pub fn add_label(&mut self, label: &str) {
        self.labels.push(label.to_string());
    }

    pub fn remove_label(&mut self, index: usize) -> Option<String> {
        if index < self.labels.len() {
            Some(self.labels.remove(index))
        } else {
            None
        }
    }

    pub fn get_labels(&self) -> Vec<String> {
        self.labels.clone()
    }
}
