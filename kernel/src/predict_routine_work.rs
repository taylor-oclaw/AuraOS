extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn predict_routine_work_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn predict_routine_work_exit() {
    // Cleanup logic for the module
}

pub struct PredictRoutineWork {
    data: Vec<u8>,
    predictions: Vec<String>,
}

impl PredictRoutineWork {
    pub fn new(data: Vec<u8>) -> Self {
        PredictRoutineWork {
            data,
            predictions: Vec::new(),
        }
    }

    pub fn add_data(&mut self, more_data: Vec<u8>) {
        self.data.extend(more_data);
    }

    pub fn get_data_length(&self) -> usize {
        self.data.len()
    }

    pub fn make_prediction(&mut self, prediction: String) {
        self.predictions.push(prediction);
    }

    pub fn get_predictions(&self) -> &Vec<String> {
        &self.predictions
    }

    pub fn clear_predictions(&mut self) {
        self.predictions.clear();
    }
}
