extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct ComputeShader {
    code: String,
    inputs: Vec<String>,
    outputs: Vec<String>,
    constants: Vec<(String, u32)>,
    execution_count: u32,
}

impl ComputeShader {
    pub fn new(code: &str) -> Self {
        ComputeShader {
            code: String::from(code),
            inputs: Vec::new(),
            outputs: Vec::new(),
            constants: Vec::new(),
            execution_count: 0,
        }
    }

    pub fn add_input(&mut self, input_name: &str) {
        self.inputs.push(String::from(input_name));
    }

    pub fn add_output(&mut self, output_name: &str) {
        self.outputs.push(String::from(output_name));
    }

    pub fn set_constant(&mut self, constant_name: &str, value: u32) {
        let index = self.constants.iter().position(|c| c.0 == constant_name);
        match index {
            Some(i) => self.constants[i] = (String::from(constant_name), value),
            None => self.constants.push((String::from(constant_name), value)),
        }
    }

    pub fn get_constant(&self, constant_name: &str) -> Option<u32> {
        self.constants.iter().find(|c| c.0 == constant_name).map(|c| c.1)
    }

    pub fn execute(&mut self) {
        // Simulate shader execution
        self.execution_count += 1;
    }
}
