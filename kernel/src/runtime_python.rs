extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct RuntimePython {
    modules: Vec<String>,
    variables: Vec<(String, String)>,
}

impl RuntimePython {
    pub fn new() -> Self {
        RuntimePython {
            modules: Vec::new(),
            variables: Vec::new(),
        }
    }

    pub fn load_module(&mut self, module_name: &str) {
        if !self.modules.contains(&module_name.to_string()) {
            self.modules.push(module_name.to_string());
        }
    }

    pub fn unload_module(&mut self, module_name: &str) {
        self.modules.retain(|m| m != module_name);
    }

    pub fn set_variable(&mut self, name: &str, value: &str) {
        let index = self.variables.iter().position(|v| v.0 == name);
        match index {
            Some(i) => self.variables[i] = (name.to_string(), value.to_string()),
            None => self.variables.push((name.to_string(), value.to_string())),
        }
    }

    pub fn get_variable(&self, name: &str) -> Option<&String> {
        self.variables.iter().find(|v| v.0 == name).map(|v| &v.1)
    }

    pub fn list_modules(&self) -> Vec<&String> {
        self.modules.iter().collect()
    }
}
