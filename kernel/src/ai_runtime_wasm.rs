extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct AiRuntimeWasm {
    data: Vec<u8>,
}

impl AiRuntimeWasm {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn add_data(&mut self, data: &[u8]) {
        self.data.extend_from_slice(data);
    }

    pub fn get_data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn size_of_data(&self) -> usize {
        self.data.len()
    }
}

pub struct AiRuntimeWasmModule {
    ai_runtime_wasm: AiRuntimeWasm,
}

impl AiRuntimeWasmModule {
    pub fn new() -> Self {
        Self { ai_runtime_wasm: AiRuntimeWasm::new() }
    }

    pub fn add_data(&mut self, data: &[u8]) {
        self.ai_runtime_wasm.add_data(data);
    }

    pub fn get_data(&self) -> &Vec<u8> {
        self.ai_runtime_wasm.get_data()
    }

    pub fn clear_data(&mut self) {
        self.ai_runtime_wasm.clear_data();
    }
}