extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    let mut tool = AgentToolMaker::new();
    tool.add_tool("tool1", "description of tool1");
    tool.add_tool("tool2", "description of tool2");
    tool.remove_tool("tool1");
    let tools = tool.list_tools();
    for tool in tools.iter() {
    }
    let description = tool.get_tool_description("tool2").unwrap_or_else(|| String::from("Tool not found"));
}

pub struct AgentToolMaker {
    tools: Vec<(String, String)>,
}

impl AgentToolMaker {
    pub fn new() -> Self {
        AgentToolMaker { tools: Vec::new() }
    }

    pub fn add_tool(&mut self, name: &str, description: &str) {
        self.tools.push((String::from(name), String::from(description)));
    }

    pub fn remove_tool(&mut self, name: &str) {
        self.tools.retain(|&(ref tool_name, _)| tool_name != name);
    }

    pub fn list_tools(&self) -> Vec<String> {
        self.tools.iter().map(|(name, _)| name.clone()).collect()
    }

    pub fn get_tool_description(&self, name: &str) -> Option<String> {
        self.tools.iter().find_map(|&(ref tool_name, ref desc)| {
            if tool_name == name {
                Some(desc.clone())
            }) else {
                None
            })
        })
    }
}
