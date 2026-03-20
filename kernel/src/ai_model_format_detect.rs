extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct AIModelFormatDetect {
    supported_formats: Vec<String>,
}

impl AIModelFormatDetect {
    pub fn new() -> Self {
        AIModelFormatDetect {
            supported_formats: vec![
                String::from("ONNX"),
                String::from("TensorFlow SavedModel"),
                String::from("PyTorch TorchScript"),
                String::from("Caffe"),
                String::from("MXNet"),
            ],
        }
    }

    pub fn is_format_supported(&self, format: &str) -> bool {
        self.supported_formats.contains(&String::from(format))
    }

    pub fn add_format_support(&mut self, format: &str) {
        if !self.is_format_supported(format) {
            self.supported_formats.push(String::from(format));
        }
    }

    pub fn remove_format_support(&mut self, format: &str) {
        self.supported_formats.retain(|f| f != format);
    }

    pub fn list_supported_formats(&self) -> Vec<String> {
        self.supported_formats.clone()
    }

    pub fn detect_format(&self, model_data: &[u8]) -> Option<&String> {
        // Simple heuristic based on file header
        if model_data.len() >= 4 && &model_data[0..4] == b"ONNX" {
            Some(&self.supported_formats[0])
        } else if model_data.len() >= 3 && &model_data[0..3] == b"TFS" {
            Some(&self.supported_formats[1])
        } else if model_data.len() >= 8 && &model_data[0..8] == b"TORCHSCR" {
            Some(&self.supported_formats[2])
        } else if model_data.len() >= 4 && &model_data[0..4] == b"CFF " {
            Some(&self.supported_formats[3])
        } else if model_data.len() >= 8 && &model_data[0..8] == b"MXNET " {
            Some(&self.supported_formats[4])
        } else {
            None
        }
    }
}
