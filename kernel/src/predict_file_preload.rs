extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn predict_file_preload_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn predict_file_preload_exit() {
    // Cleanup logic for the module
}

pub struct PredictFilePreload {
    predictions: Vec<String>,
}

impl PredictFilePreload {
    pub fn new() -> Self {
        PredictFilePreload {
            predictions: Vec::new(),
        }
    }

    pub fn add_prediction(&mut self, prediction: String) {
        self.predictions.push(prediction);
    }

    pub fn remove_prediction(&mut self, index: usize) -> Option<String> {
        if index < self.predictions.len() {
            Some(self.predictions.remove(index))
        } else {
            None
        }
    }

    pub fn get_predictions(&self) -> &Vec<String> {
        &self.predictions
    }

    pub fn clear_predictions(&mut self) {
        self.predictions.clear();
    }

    pub fn has_prediction(&self, prediction: &str) -> bool {
        self.predictions.iter().any(|p| p == prediction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_predict_file_preload() {
        let mut preload = PredictFilePreload::new();
        assert!(preload.get_predictions().is_empty());

        preload.add_prediction(String::from("file1"));
        preload.add_prediction(String::from("file2"));
        assert_eq!(preload.get_predictions().len(), 2);

        assert!(preload.has_prediction("file1"));
        assert!(!preload.has_prediction("file3"));

        let removed = preload.remove_prediction(0);
        assert_eq!(removed, Some(String::from("file1")));
        assert_eq!(preload.get_predictions().len(), 1);

        preload.clear_predictions();
        assert!(preload.get_predictions().is_empty());
    }
}
