extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut accelerator = AiAcceleratorAbstract::new();
    accelerator.initialize();
    loop {}
}

pub struct AiAcceleratorAbstract {
    name: String,
    capabilities: Vec<String>,
    initialized: bool,
}

impl AiAcceleratorAbstract {
    pub fn new() -> Self {
        AiAcceleratorAbstract {
            name: String::from("AI-Accelerator"),
            capabilities: Vec::new(),
            initialized: false,
        }
    }

    pub fn initialize(&mut self) {
        if !self.initialized {
            self.capabilities.push(String::from("Inference"));
            self.capabilities.push(String::from("Training"));
            self.capabilities.push(String::from("Optimization"));
            self.capabilities.push(String::from("Quantization"));
            self.capabilities.push(String::from("Debugging"));
            self.initialized = true;
        }
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn list_capabilities(&self) -> Vec<&str> {
        self.capabilities.iter().map(|c| c.as_str()).collect()
    }
}
