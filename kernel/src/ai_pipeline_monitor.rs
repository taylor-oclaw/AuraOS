extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    let monitor = AIPipelineMonitor::new();
    monitor.start_pipeline("model1");
    monitor.log_status();
    monitor.stop_pipeline("model2");
    monitor.add_model("model3");
    monitor.remove_model("model2");
}

pub struct AIPipelineMonitor {
    models: Vec<String>,
    active_pipelines: Vec<String>,
}

impl AIPipelineMonitor {
    pub fn new() -> Self {
        AIPipelineMonitor {
            models: Vec::new(),
            active_pipelines: Vec::new(),
        }
    }

    pub fn add_model(&mut self, model_name: &str) {
        if !self.models.contains(&model_name.to_string()) {
            self.models.push(model_name.to_string());
        }
    }

    pub fn remove_model(&mut self, model_name: &str) {
        self.models.retain(|m| m != model_name);
        self.active_pipelines.retain(|p| p != model_name);
    }

    pub fn start_pipeline(&mut self, model_name: &str) {
        if self.models.contains(&model_name.to_string()) && !self.active_pipelines.contains(&model_name.to_string()) {
            self.active_pipelines.push(model_name.to_string());
        }
    }

    pub fn stop_pipeline(&mut self, model_name: &str) {
        self.active_pipelines.retain(|p| p != model_name);
    }

    pub fn log_status(&self) {
        for model in &self.models {
            if self.active_pipelines.contains(model) {
                // Simulate logging active pipeline
            } else {
                // Simulate logging inactive pipeline
            }
        }
    }
}
