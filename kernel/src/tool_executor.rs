extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ToolExecutor {
    tools: Vec<String>,
}

impl ToolExecutor {
    pub fn new() -> Self {
        ToolExecutor { tools: Vec::new() }
    }

    pub fn add_tool(&mut self, tool_name: &str) {
        self.tools.push(String::from(tool_name));
    }

    pub fn remove_tool(&mut self, tool_name: &str) {
        if let Some(index) = self.tools.iter().position(|t| t == tool_name) {
            self.tools.remove(index);
        }
    }

    pub fn list_tools(&self) -> Vec<String> {
        self.tools.clone()
    }

    pub fn has_tool(&self, tool_name: &str) -> bool {
        self.tools.contains(&String::from(tool_name))
    }

    pub fn execute_tool(&self, tool_name: &str) -> Option<&String> {
        self.tools.iter().find(|t| t.as_str() == tool_name)
    }
}
