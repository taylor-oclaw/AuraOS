extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct PredictWorkflowDetect {
    workflows: Vec<String>,
    predictions: Vec<f32>,
}

impl PredictWorkflowDetect {
    pub fn new() -> Self {
        PredictWorkflowDetect {
            workflows: Vec::new(),
            predictions: Vec::new(),
        }
    }

    pub fn add_workflow(&mut self, workflow: String) {
        self.workflows.push(workflow);
    }

    pub fn get_workflows(&self) -> &Vec<String> {
        &self.workflows
    }

    pub fn predict(&mut self, index: usize) -> Option<f32> {
        if index < self.workflows.len() {
            let prediction = self.calculate_prediction(index);
            self.predictions.push(prediction);
            Some(prediction)
        } else {
            None
        }
    }

    pub fn get_predictions(&self) -> &Vec<f32> {
        &self.predictions
    }

    pub fn clear_data(&mut self) {
        self.workflows.clear();
        self.predictions.clear();
    }

    fn calculate_prediction(&self, index: usize) -> f32 {
        // Dummy prediction logic for demonstration purposes
        let workflow_length = self.workflows[index].len() as f32;
        0.5 * (workflow_length / 10.0)
    }
}
