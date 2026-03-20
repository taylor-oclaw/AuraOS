extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut pipeline = AIPipelineReplay::new();
    pipeline.add_step(String::from("Data Ingestion"));
    pipeline.add_step(String::from("Preprocessing"));
    pipeline.add_step(String::from("Model Inference"));
    pipeline.add_step(String::from("Postprocessing"));
    pipeline.add_step(String::from("Output Generation"));

    pipeline.execute_pipeline();
}

pub struct AIPipelineReplay {
    steps: Vec<String>,
    current_step_index: usize,
}

impl AIPipelineReplay {
    pub fn new() -> Self {
        AIPipelineReplay {
            steps: Vec::new(),
            current_step_index: 0,
        }
    }

    pub fn add_step(&mut self, step_name: String) {
        self.steps.push(step_name);
    }

    pub fn get_current_step(&self) -> Option<&String> {
        if self.current_step_index < self.steps.len() {
            Some(&self.steps[self.current_step_index])
        } else {
            None
        }
    }

    pub fn execute_pipeline(&mut self) {
        while let Some(step) = self.get_current_step() {
            println!("Executing step: {}", step);
            self.move_to_next_step();
        }
    }

    pub fn move_to_next_step(&mut self) {
        if self.current_step_index < self.steps.len() {
            self.current_step_index += 1;
        }
    }

    pub fn reset_pipeline(&mut self) {
        self.current_step_index = 0;
    }
}
