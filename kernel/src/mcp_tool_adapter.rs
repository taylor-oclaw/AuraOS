extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn mcp_tool_adapter_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn mcp_tool_adapter_exit() {
    // Cleanup logic for the module
}

pub struct McpToolAdapter {
    data: Vec<u8>,
    name: String,
}

impl McpToolAdapter {
    pub fn new(name: &str) -> Self {
        McpToolAdapter {
            data: Vec::new(),
            name: String::from(name),
        }
    }

    pub fn add_data(&mut self, bytes: &[u8]) {
        self.data.extend_from_slice(bytes);
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}
