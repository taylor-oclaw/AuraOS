extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut stream = AIInferenceStream::new();
    stream.add_model("model1");
    stream.add_data(String::from("data1"));
    stream.process_data();
    let result = stream.get_result();
    loop {}
}

pub struct AIInferenceStream {
    models: Vec<String>,
    data: Vec<String>,
    results: Vec<String>,
}

impl AIInferenceStream {
    pub fn new() -> Self {
        AIInferenceStream {
            models: Vec::new(),
            data: Vec::new(),
            results: Vec::new(),
        }
    }

    pub fn add_model(&mut self, model_name: &str) {
        self.models.push(model_name.to_string());
    }

    pub fn add_data(&mut self, data: String) {
        self.data.push(data);
    }

    pub fn process_data(&mut self) {
        for d in &self.data {
            // Simulate processing data with a model
            let result = String::from("info");
            self.results.push(result);
        }
    }

    pub fn get_result(&self) -> String {
        if let Some(last_result) = self.results.last() {
            last_result.clone()
        } else {
            String::from("No results")
        }
    }

    pub fn clear_data(&mut self) {
        self.data.clear();
    }
}
