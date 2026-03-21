extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut detector = SpeechDysarthriaDetector::new();
    
    // Example usage of the methods
    detector.add_sample(String::from("slurred speech"));
    detector.add_sample(String::from("clear speech"));
    detector.analyze_samples();
    let result = detector.get_analysis_result();
    println!("Analysis Result: {}", result);
}

pub struct SpeechDysarthriaDetector {
    samples: Vec<String>,
    analysis_result: String,
}

impl SpeechDysarthriaDetector {
    pub fn new() -> Self {
        SpeechDysarthriaDetector {
            samples: Vec::new(),
            analysis_result: String::from("No analysis performed"),
        }
    }

    pub fn add_sample(&mut self, sample: String) {
        self.samples.push(sample);
    }

    pub fn remove_sample(&mut self, index: usize) -> Option<String> {
        if index < self.samples.len() {
            Some(self.samples.remove(index))
        } else {
            None
        }
    }

    pub fn get_samples_count(&self) -> usize {
        self.samples.len()
    }

    pub fn analyze_samples(&mut self) {
        // Simple analysis logic: count slurred speech samples
        let slurred_count = self.samples.iter().filter(|&s| s.contains("slurred")).count();
        let total_count = self.samples.len();
        
        if total_count == 0 {
            self.analysis_result = String::from("No samples to analyze");
        } else {
            let percentage = (slurred_count as f32 / total_count as f32) * 100.0;
            self.analysis_result = format!("Slurred speech: {}% of {} samples", percentage, total_count);
        }
    }

    pub fn get_analysis_result(&self) -> &str {
        &self.analysis_result
    }
}
