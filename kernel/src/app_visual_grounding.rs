extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn app_visual_grounding_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn app_visual_grounding_exit() {
    // Cleanup logic for the module
}

pub struct AppVisualGrounding {
    data: Vec<u8>,
    labels: Vec<String>,
}

impl AppVisualGrounding {
    pub fn new() -> Self {
        AppVisualGrounding {
            data: Vec::new(),
            labels: Vec::new(),
        }
    }

    pub fn add_data(&mut self, data: u8) {
        self.data.push(data);
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    pub fn add_label(&mut self, label: String) {
        self.labels.push(label);
    }

    pub fn get_labels(&self) -> &[String] {
        &self.labels
    }

    pub fn clear_data(&mut self) {
        self.data.clear();
    }
}
